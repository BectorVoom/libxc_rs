//! GGA_X_PBEPOW vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbepow_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t42 = f64x8::splat(0.9146457198521546) * t40 + f64x8::splat(0.804);
            let t43 = f64x8::splat(1.0) / t42;
            let t45 = t33 * t39 * t43;
            let t46 = (simd::pow(t45, f64x8::splat(100.0)));
            let t48 = f64x8::splat(0.0001334414156799501) * t46 - f64x8::splat(1.0);
            let t52 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t33 * t39 * t48;
            let t56 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t52));
            let t57 = (v_rho1).simd_le(dens_threshold);
            let t58 = -t16;
            let t60 = ((t14).select(t11, (t10).select(t15, t58 * t7)));
            let t61 = f64x8::splat(1.0) + t60;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t22, t63 * t61));
            let t66 = t65 * t26;
            let t67 = v_rho1 * v_rho1;
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t68 * t68;
            let t71 = f64x8::splat(1.0) / t69 / t67;
            let t72 = v_sigma2 * t71;
            let t73 = t33 * t72;
            let t75 = f64x8::splat(0.9146457198521546) * t73 + f64x8::splat(0.804);
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t33 * t72 * t76;
            let t79 = (simd::pow(t78, f64x8::splat(100.0)));
            let t81 = f64x8::splat(0.0001334414156799501) * t79 - f64x8::splat(1.0);
            let t85 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t33 * t72 * t81;
            let t89 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t85));
            let tzk0 = t56 + t89;
            acc_zk = tzk0;
            let t90 = t6 * t6;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t16 * t91;
            let t94 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t92)));
            let t97 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t94));
            let t98 = t97 * t26;
            let t102 = t26 * t26;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t25 * t103;
            let t107 = t5 * t104 * t52 / f64x8::splat(8.0);
            let t108 = t34 * v_rho0;
            let t110 = f64x8::splat(1.0) / t36 / t108;
            let t111 = v_sigma0 * t110;
            let t115 = t33 * v_sigma0;
            let t116 = (simd::pow(t45, f64x8::splat(99.0)));
            let t117 = t38 * t116;
            let t121 = t28 * t28;
            let t123 = f64x8::splat(1.0) / t30 / t29;
            let t124 = t121 * t123;
            let t125 = v_sigma0 * v_sigma0;
            let t126 = t34 * t34;
            let t127 = t126 * t34;
            let t129 = f64x8::splat(1.0) / t35 / t127;
            let t131 = t42 * t42;
            let t132 = f64x8::splat(1.0) / t131;
            let t136 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t111 * t43 + f64x8::splat(2.4390552529390788) * t124 * t125 * t129 * t132;
            let t137 = t117 * t136;
            let t140 = f64x8::splat(0.024390552529390788) * t33 * t111 * t48 - f64x8::splat(0.00012205161970267855) * t115 * t137;
            let t145 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t98 * t52 - t107 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t140));
            let t146 = t58 * t91;
            let t148 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t146)));
            let t151 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t148));
            let t152 = t151 * t26;
            let t156 = t65 * t103;
            let t159 = t5 * t156 * t85 / f64x8::splat(8.0);
            let t161 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t85 - t159));
            let tvrho0 = t56 + t89 + t6 * (t145 + t161);
            acc_vrho_0 = tvrho0;
            let t165 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t92)));
            let t168 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t165));
            let t169 = t168 * t26;
            let t174 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t52 - t107));
            let t176 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t146)));
            let t179 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t176));
            let t180 = t179 * t26;
            let t184 = t67 * v_rho1;
            let t186 = f64x8::splat(1.0) / t69 / t184;
            let t187 = v_sigma2 * t186;
            let t191 = t33 * v_sigma2;
            let t192 = (simd::pow(t78, f64x8::splat(99.0)));
            let t193 = t71 * t192;
            let t197 = v_sigma2 * v_sigma2;
            let t198 = t67 * t67;
            let t199 = t198 * t67;
            let t201 = f64x8::splat(1.0) / t68 / t199;
            let t203 = t75 * t75;
            let t204 = f64x8::splat(1.0) / t203;
            let t208 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t187 * t76 + f64x8::splat(2.4390552529390788) * t124 * t197 * t201 * t204;
            let t209 = t193 * t208;
            let t212 = f64x8::splat(0.024390552529390788) * t33 * t187 * t81 - f64x8::splat(0.00012205161970267855) * t191 * t209;
            let t217 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t85 - t159 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t212));
            let tvrho1 = t56 + t89 + t6 * (t174 + t217);
            acc_vrho_1 = tvrho1;
            let t225 = t126 * v_rho0;
            let t227 = f64x8::splat(1.0) / t35 / t225;
            let t232 = t33 * t38 * t43 - f64x8::splat(0.9146457198521546) * t124 * v_sigma0 * t227 * t132;
            let t233 = t117 * t232;
            let t236 = -f64x8::splat(0.009146457198521547) * t33 * t38 * t48 - f64x8::splat(0.00012205161970267855) * t115 * t233;
            let t240 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t236));
            let tvsigma0 = t6 * t240;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t246 = t198 * v_rho1;
            let t248 = f64x8::splat(1.0) / t68 / t246;
            let t253 = t33 * t71 * t76 - f64x8::splat(0.9146457198521546) * t124 * v_sigma2 * t248 * t204;
            let t254 = t193 * t253;
            let t257 = -f64x8::splat(0.009146457198521547) * t33 * t71 * t81 - f64x8::splat(0.00012205161970267855) * t191 * t254;
            let t261 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t257));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
