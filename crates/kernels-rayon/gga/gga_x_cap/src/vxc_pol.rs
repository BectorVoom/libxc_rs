//! GGA_X_CAP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`
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
pub fn gga_x_cap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alphaoAx: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphaoAx = f64x8::splat(param_alphaoAx);
    let param_c = f64x8::splat(param_c);
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
            let t29 = t28 * t28;
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = param_alphaoAx * t29 * t33;
            let t35 = ((v_sigma0).sqrt());
            let t36 = (simd::cbrt(v_rho0));
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t39 = t35 * t38;
            let t40 = t29 * t33;
            let t43 = f64x8::splat(1.0) + t40 * t39 / f64x8::splat(12.0);
            let t44 = (simd::ln(t43));
            let t46 = param_c * t44 + f64x8::splat(1.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t44 * t47;
            let t52 = f64x8::splat(1.0) - t34 * t39 * t48 / f64x8::splat(12.0);
            let t56 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t52));
            let t57 = (v_rho1).simd_le(dens_threshold);
            let t58 = -t16;
            let t60 = ((t14).select(t11, (t10).select(t15, t58 * t7)));
            let t61 = f64x8::splat(1.0) + t60;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t22, t63 * t61));
            let t66 = t65 * t26;
            let t67 = ((v_sigma2).sqrt());
            let t68 = (simd::cbrt(v_rho1));
            let t70 = f64x8::splat(1.0) / t68 / v_rho1;
            let t71 = t67 * t70;
            let t74 = f64x8::splat(1.0) + t40 * t71 / f64x8::splat(12.0);
            let t75 = (simd::ln(t74));
            let t77 = param_c * t75 + f64x8::splat(1.0);
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t75 * t78;
            let t83 = f64x8::splat(1.0) - t34 * t71 * t79 / f64x8::splat(12.0);
            let t87 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t83));
            let tzk0 = t56 + t87;
            acc_zk = tzk0;
            let t88 = t6 * t6;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t16 * t89;
            let t92 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t90)));
            let t95 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t92));
            let t96 = t95 * t26;
            let t100 = t26 * t26;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t25 * t101;
            let t105 = t5 * t102 * t52 / f64x8::splat(8.0);
            let t106 = v_rho0 * v_rho0;
            let t108 = f64x8::splat(1.0) / t36 / t106;
            let t113 = param_alphaoAx * t28;
            let t114 = t32 * t32;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t113 * t115;
            let t117 = t106 * v_rho0;
            let t118 = t36 * t36;
            let t120 = f64x8::splat(1.0) / t118 / t117;
            let t122 = f64x8::splat(1.0) / t43;
            let t123 = t122 * t47;
            let t128 = t113 * t115 * v_sigma0;
            let t130 = t46 * t46;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t131 * param_c;
            let t133 = t132 * t122;
            let t134 = t120 * t44 * t133;
            let t137 = t34 * t35 * t108 * t48 / f64x8::splat(9.0) + t116 * v_sigma0 * t120 * t123 / f64x8::splat(18.0) - t128 * t134 / f64x8::splat(18.0);
            let t142 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t96 * t52 - t105 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t137));
            let t143 = t58 * t89;
            let t145 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t143)));
            let t148 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t145));
            let t149 = t148 * t26;
            let t153 = t65 * t101;
            let t156 = t5 * t153 * t83 / f64x8::splat(8.0);
            let t158 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t83 - t156));
            let tvrho0 = t56 + t87 + t6 * (t142 + t158);
            acc_vrho_0 = tvrho0;
            let t162 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t90)));
            let t165 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t162));
            let t166 = t165 * t26;
            let t171 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t52 - t105));
            let t173 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t143)));
            let t176 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t173));
            let t177 = t176 * t26;
            let t181 = v_rho1 * v_rho1;
            let t183 = f64x8::splat(1.0) / t68 / t181;
            let t188 = t181 * v_rho1;
            let t189 = t68 * t68;
            let t191 = f64x8::splat(1.0) / t189 / t188;
            let t193 = f64x8::splat(1.0) / t74;
            let t194 = t193 * t78;
            let t199 = t113 * t115 * v_sigma2;
            let t201 = t77 * t77;
            let t202 = f64x8::splat(1.0) / t201;
            let t203 = t202 * param_c;
            let t204 = t203 * t193;
            let t205 = t191 * t75 * t204;
            let t208 = t34 * t67 * t183 * t79 / f64x8::splat(9.0) + t116 * v_sigma2 * t191 * t194 / f64x8::splat(18.0) - t199 * t205 / f64x8::splat(18.0);
            let t213 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t177 * t83 - t156 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t208));
            let tvrho1 = t56 + t87 + t6 * (t171 + t213);
            acc_vrho_1 = tvrho1;
            let t216 = f64x8::splat(1.0) / t35;
            let t222 = f64x8::splat(1.0) / t118 / t106;
            let t229 = t44 * t131;
            let t231 = t229 * param_c * t122;
            let t234 = -t34 * t216 * t38 * t48 / f64x8::splat(24.0) - t116 * t222 * t122 * t47 / f64x8::splat(48.0) + t113 * t115 * t222 * t231 / f64x8::splat(48.0);
            let t238 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t234));
            let tvsigma0 = t6 * t238;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t239 = f64x8::splat(1.0) / t67;
            let t245 = f64x8::splat(1.0) / t189 / t181;
            let t252 = t75 * t202;
            let t254 = t252 * param_c * t193;
            let t257 = -t34 * t239 * t70 * t79 / f64x8::splat(24.0) - t116 * t245 * t193 * t78 / f64x8::splat(48.0) + t113 * t115 * t245 * t254 / f64x8::splat(48.0);
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
