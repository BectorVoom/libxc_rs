//! GGA_X_PBETRANS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
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
pub fn gga_x_pbetrans_vxc_pol(
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
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t2 * t29;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = t31 * t31;
            let t34 = t32 / t29;
            let t35 = ((v_sigma0).sqrt());
            let t36 = (simd::cbrt(v_rho0));
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t45 = (simd::exp(-f64x8::splat(2.0) * t30 * (t34 * t35 * t38 / f64x8::splat(12.0) - f64x8::splat(3.0))));
            let t46 = f64x8::splat(1.0) + t45;
            let t48 = f64x8::splat(0.413) / t46;
            let t49 = f64x8::splat(1.227) - t48;
            let t50 = t29 * t29;
            let t52 = t31 / t50;
            let t53 = v_rho0 * v_rho0;
            let t54 = t36 * t36;
            let t56 = f64x8::splat(1.0) / t54 / t53;
            let t60 = f64x8::splat(1.227) - t48 + f64x8::splat(0.009125) * t52 * v_sigma0 * t56;
            let t61 = f64x8::splat(1.0) / t60;
            let t63 = -t49 * t61 + f64x8::splat(1.0);
            let t65 = t49 * t63 + f64x8::splat(1.0);
            let t69 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t65));
            let t70 = (v_rho1).simd_le(dens_threshold);
            let t71 = -t16;
            let t73 = ((t14).select(t11, (t10).select(t15, t71 * t7)));
            let t74 = f64x8::splat(1.0) + t73;
            let t75 = (t74).simd_le(zeta_threshold);
            let t76 = (simd::cbrt(t74));
            let t78 = ((t75).select(t22, t76 * t74));
            let t79 = t78 * t26;
            let t80 = ((v_sigma2).sqrt());
            let t81 = (simd::cbrt(v_rho1));
            let t83 = f64x8::splat(1.0) / t81 / v_rho1;
            let t90 = (simd::exp(-f64x8::splat(2.0) * t30 * (t34 * t80 * t83 / f64x8::splat(12.0) - f64x8::splat(3.0))));
            let t91 = f64x8::splat(1.0) + t90;
            let t93 = f64x8::splat(0.413) / t91;
            let t94 = f64x8::splat(1.227) - t93;
            let t95 = v_rho1 * v_rho1;
            let t96 = t81 * t81;
            let t98 = f64x8::splat(1.0) / t96 / t95;
            let t102 = f64x8::splat(1.227) - t93 + f64x8::splat(0.009125) * t52 * v_sigma2 * t98;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = -t94 * t103 + f64x8::splat(1.0);
            let t107 = t94 * t105 + f64x8::splat(1.0);
            let t111 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t79 * t107));
            let tzk0 = t69 + t111;
            acc_zk = tzk0;
            let t112 = t6 * t6;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t16 * t113;
            let t116 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t114)));
            let t119 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t116));
            let t120 = t119 * t26;
            let t124 = t26 * t26;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t25 * t125;
            let t129 = t5 * t126 * t65 / f64x8::splat(8.0);
            let t130 = t46 * t46;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t131 * t2;
            let t133 = t132 * t32;
            let t135 = f64x8::splat(1.0) / t36 / t53;
            let t136 = t35 * t135;
            let t137 = t45 * t63;
            let t141 = t45 * t61;
            let t145 = t60 * t60;
            let t146 = f64x8::splat(1.0) / t145;
            let t147 = t49 * t146;
            let t151 = t53 * v_rho0;
            let t153 = f64x8::splat(1.0) / t54 / t151;
            let t157 = f64x8::splat(0.09177777777777778) * t133 * t136 * t45 - f64x8::splat(0.024333333333333332) * t52 * v_sigma0 * t153;
            let t159 = -f64x8::splat(0.09177777777777778) * t133 * t136 * t141 + t147 * t157;
            let t161 = f64x8::splat(0.09177777777777778) * t133 * t136 * t137 + t49 * t159;
            let t166 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t120 * t65 - t129 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t161));
            let t167 = t71 * t113;
            let t169 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t167)));
            let t172 = ((t75).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t76 * t169));
            let t173 = t172 * t26;
            let t177 = t78 * t125;
            let t180 = t5 * t177 * t107 / f64x8::splat(8.0);
            let t182 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t173 * t107 - t180));
            let tvrho0 = t69 + t111 + t6 * (t166 + t182);
            acc_vrho_0 = tvrho0;
            let t186 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t114)));
            let t189 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t186));
            let t190 = t189 * t26;
            let t195 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t190 * t65 - t129));
            let t197 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t167)));
            let t200 = ((t75).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t76 * t197));
            let t201 = t200 * t26;
            let t205 = t91 * t91;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t206 * t2;
            let t208 = t207 * t32;
            let t210 = f64x8::splat(1.0) / t81 / t95;
            let t211 = t80 * t210;
            let t212 = t90 * t105;
            let t216 = t90 * t103;
            let t220 = t102 * t102;
            let t221 = f64x8::splat(1.0) / t220;
            let t222 = t94 * t221;
            let t226 = t95 * v_rho1;
            let t228 = f64x8::splat(1.0) / t96 / t226;
            let t232 = f64x8::splat(0.09177777777777778) * t208 * t211 * t90 - f64x8::splat(0.024333333333333332) * t52 * v_sigma2 * t228;
            let t234 = -f64x8::splat(0.09177777777777778) * t208 * t211 * t216 + t222 * t232;
            let t236 = f64x8::splat(0.09177777777777778) * t208 * t211 * t212 + t94 * t234;
            let t241 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t201 * t107 - t180 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t79 * t236));
            let tvrho1 = t69 + t111 + t6 * (t195 + t241);
            acc_vrho_1 = tvrho1;
            let t244 = f64x8::splat(1.0) / t35;
            let t245 = t244 * t38;
            let t257 = -f64x8::splat(0.034416666666666665) * t133 * t245 * t45 + f64x8::splat(0.009125) * t52 * t56;
            let t259 = f64x8::splat(0.034416666666666665) * t133 * t245 * t141 + t147 * t257;
            let t261 = -f64x8::splat(0.034416666666666665) * t133 * t245 * t137 + t49 * t259;
            let t265 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t261));
            let tvsigma0 = t6 * t265;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t266 = f64x8::splat(1.0) / t80;
            let t267 = t266 * t83;
            let t279 = -f64x8::splat(0.034416666666666665) * t208 * t267 * t90 + f64x8::splat(0.009125) * t52 * t98;
            let t281 = f64x8::splat(0.034416666666666665) * t208 * t267 * t216 + t222 * t279;
            let t283 = -f64x8::splat(0.034416666666666665) * t208 * t267 * t212 + t94 * t281;
            let t287 = ((t70).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t79 * t283));
            let tvsigma2 = t6 * t287;
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
