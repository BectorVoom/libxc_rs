//! GGA_K_EXP4 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_exp4_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t34;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_rho0 * v_rho0;
            let t39 = (simd::cbrt(v_rho0));
            let t40 = t39 * t39;
            let t42 = f64x8::splat(1.0) / t40 / t38;
            let t46 = (simd::exp(-f64x8::splat(8.325416666666667) * t37 * v_sigma0 * t42));
            let t48 = t32 * t32;
            let t51 = t48 / t34 / t33;
            let t52 = v_sigma0 * v_sigma0;
            let t53 = t38 * t38;
            let t54 = t53 * v_rho0;
            let t56 = f64x8::splat(1.0) / t39 / t54;
            let t60 = (simd::exp(-f64x8::splat(0.007547916666666666) * t51 * t52 * t56));
            let t62 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t46 - f64x8::splat(1.2264) * t60;
            let t66 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t62));
            let t67 = (v_rho1).simd_le(dens_threshold);
            let t68 = -t17;
            let t70 = ((t15).select(t12, (t11).select(t16, t68 * t8)));
            let t71 = f64x8::splat(1.0) + t70;
            let t72 = (t71).simd_le(zeta_threshold);
            let t73 = (simd::cbrt(t71));
            let t74 = t73 * t73;
            let t76 = ((t72).select(t24, t74 * t71));
            let t77 = t76 * t30;
            let t78 = v_rho1 * v_rho1;
            let t79 = (simd::cbrt(v_rho1));
            let t80 = t79 * t79;
            let t82 = f64x8::splat(1.0) / t80 / t78;
            let t86 = (simd::exp(-f64x8::splat(8.325416666666667) * t37 * v_sigma2 * t82));
            let t88 = v_sigma2 * v_sigma2;
            let t89 = t78 * t78;
            let t90 = t89 * v_rho1;
            let t92 = f64x8::splat(1.0) / t79 / t90;
            let t96 = (simd::exp(-f64x8::splat(0.007547916666666666) * t51 * t88 * t92));
            let t98 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t86 - f64x8::splat(1.2264) * t96;
            let t102 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t98));
            let tzk0 = t66 + t102;
            acc_zk = tzk0;
            let t103 = t7 * t7;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t17 * t104;
            let t107 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t105)));
            let t110 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t107));
            let t111 = t110 * t30;
            let t115 = f64x8::splat(1.0) / t29;
            let t116 = t28 * t115;
            let t119 = t6 * t116 * t62 / f64x8::splat(10.0);
            let t120 = t38 * v_rho0;
            let t122 = f64x8::splat(1.0) / t40 / t120;
            let t127 = t53 * t38;
            let t129 = f64x8::splat(1.0) / t39 / t127;
            let t134 = -f64x8::splat(18.92422711111111) * t37 * v_sigma0 * t122 * t46 - f64x8::splat(0.049369413333333334) * t51 * t52 * t129 * t60;
            let t139 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t111 * t62 + t119 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t134));
            let t140 = t68 * t104;
            let t142 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t140)));
            let t145 = ((t72).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t142));
            let t146 = t145 * t30;
            let t150 = t76 * t115;
            let t153 = t6 * t150 * t98 / f64x8::splat(10.0);
            let t155 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t98 + t153));
            let tvrho0 = t66 + t102 + t7 * (t139 + t155);
            acc_vrho_0 = tvrho0;
            let t159 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t105)));
            let t162 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t159));
            let t163 = t162 * t30;
            let t168 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t62 + t119));
            let t170 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t140)));
            let t173 = ((t72).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t170));
            let t174 = t173 * t30;
            let t178 = t78 * v_rho1;
            let t180 = f64x8::splat(1.0) / t80 / t178;
            let t185 = t89 * t78;
            let t187 = f64x8::splat(1.0) / t79 / t185;
            let t192 = -f64x8::splat(18.92422711111111) * t37 * v_sigma2 * t180 * t86 - f64x8::splat(0.049369413333333334) * t51 * t88 * t187 * t96;
            let t197 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t174 * t98 + t153 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t192));
            let tvrho1 = t66 + t102 + t7 * (t168 + t197);
            acc_vrho_1 = tvrho1;
            let t207 = f64x8::splat(7.096585166666666) * t37 * t42 * t46 + f64x8::splat(0.01851353) * t51 * v_sigma0 * t56 * t60;
            let t211 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t207));
            let tvsigma0 = t7 * t211;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t219 = f64x8::splat(7.096585166666666) * t37 * t82 * t86 + f64x8::splat(0.01851353) * t51 * v_sigma2 * t92 * t96;
            let t223 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t219));
            let tvsigma2 = t7 * t223;
            acc_vsigma_2 = tvsigma2;
            let t226 = f64x8::splat(1.0) / t25;
            let t227 = t107 * t107;
            let t230 = t103 * t7;
            let t231 = f64x8::splat(1.0) / t230;
            let t232 = t17 * t231;
            let t235 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t104 + f64x8::splat(2.0) * t232)));
            let t239 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t226 * t227 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t235));
            let t240 = t239 * t30;
            let t244 = t110 * t115;
            let t246 = t6 * t244 * t62;
            let t252 = f64x8::splat(1.0) / t29 / t7;
            let t253 = t28 * t252;
            let t256 = t6 * t253 * t62 / f64x8::splat(30.0);
            let t258 = t6 * t116 * t134;
            let t261 = f64x8::splat(1.0) / t40 / t53;
            let t266 = t53 * t120;
            let t268 = f64x8::splat(1.0) / t39 / t266;
            let t269 = t52 * t268;
            let t276 = t33 * t33;
            let t279 = t32 / t35 / t276;
            let t280 = t52 * t52;
            let t281 = t53 * t53;
            let t282 = t281 * t53;
            let t284 = f64x8::splat(1.0) / t40 / t282;
            let t289 = f64x8::splat(69.38883274074074) * t37 * v_sigma0 * t261 * t46 - f64x8::splat(420.138868785679) * t51 * t269 * t46 + f64x8::splat(0.3126729511111111) * t51 * t269 * t60 - f64x8::splat(0.011924358967111111) * t279 * t280 * t284 * t60;
            let t294 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t240 * t62 + t246 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t111 * t134 - t256 + t258 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t289));
            let t295 = f64x8::splat(1.0) / t73;
            let t296 = t142 * t142;
            let t299 = t68 * t231;
            let t302 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t104 + f64x8::splat(2.0) * t299)));
            let t306 = ((t72).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t295 * t296 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t302));
            let t307 = t306 * t30;
            let t311 = t145 * t115;
            let t313 = t6 * t311 * t98;
            let t315 = t76 * t252;
            let t318 = t6 * t315 * t98 / f64x8::splat(30.0);
            let t320 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t307 * t98 + t313 / f64x8::splat(5.0) - t318));
            let tv2rho20 = f64x8::splat(2.0) * t139 + f64x8::splat(2.0) * t155 + t7 * (t294 + t320);
            acc_v2rho2_0 = tv2rho20;
            let t323 = t226 * t159;
            let t327 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t232)));
            let t331 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t323 * t107 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t327));
            let t332 = t331 * t30;
            let t336 = t162 * t115;
            let t338 = t6 * t336 * t62;
            let t346 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t332 * t62 + t338 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t134 + t246 / f64x8::splat(10.0) - t256 + t258 / f64x8::splat(10.0)));
            let t347 = t295 * t170;
            let t351 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t299)));
            let t355 = ((t72).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t347 * t142 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t351));
            let t356 = t355 * t30;
            let t360 = t173 * t115;
            let t362 = t6 * t360 * t98;
            let t369 = t6 * t150 * t192;
            let t372 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t356 * t98 + t362 / f64x8::splat(10.0) + t313 / f64x8::splat(10.0) - t318 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t192 + t369 / f64x8::splat(10.0)));
            let tv2rho21 = t139 + t155 + t168 + t197 + t7 * (t346 + t372);
            acc_v2rho2_1 = tv2rho21;
            let t377 = t159 * t159;
            let t382 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t104 + f64x8::splat(2.0) * t232)));
            let t386 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t226 * t377 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t382));
            let t387 = t386 * t30;
            let t393 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t387 * t62 + t338 / f64x8::splat(5.0) - t256));
            let t394 = t170 * t170;
            let t399 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t104 + f64x8::splat(2.0) * t299)));
            let t403 = ((t72).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t295 * t394 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t399));
            let t404 = t403 * t30;
            let t414 = f64x8::splat(1.0) / t80 / t89;
            let t419 = t89 * t178;
            let t421 = f64x8::splat(1.0) / t79 / t419;
            let t422 = t88 * t421;
            let t429 = t88 * t88;
            let t430 = t89 * t89;
            let t431 = t430 * t89;
            let t433 = f64x8::splat(1.0) / t80 / t431;
            let t438 = f64x8::splat(69.38883274074074) * t37 * v_sigma2 * t414 * t86 - f64x8::splat(420.138868785679) * t51 * t422 * t86 + f64x8::splat(0.3126729511111111) * t51 * t422 * t96 - f64x8::splat(0.011924358967111111) * t279 * t429 * t433 * t96;
            let t443 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t404 * t98 + t362 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t174 * t192 - t318 + t369 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t438));
            let tv2rho22 = f64x8::splat(2.0) * t168 + f64x8::splat(2.0) * t197 + t7 * (t393 + t443);
            acc_v2rho2_2 = tv2rho22;
            let t451 = t6 * t116 * t207 / f64x8::splat(10.0);
            let t455 = t129 * v_sigma0;
            let t462 = t52 * v_sigma0;
            let t463 = t281 * t120;
            let t465 = f64x8::splat(1.0) / t40 / t463;
            let t470 = -f64x8::splat(18.92422711111111) * t37 * t122 * t46 + f64x8::splat(157.55207579462962) * t51 * t455 * t46 - f64x8::splat(0.09873882666666667) * t51 * t455 * t60 + f64x8::splat(0.004471634612666667) * t279 * t462 * t465 * t60;
            let t475 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t111 * t207 + t451 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t470));
            let tv2rhosigma0 = t7 * t475 + t211;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t482 = t6 * t150 * t219 / f64x8::splat(10.0);
            let t484 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t219 + t482));
            let tv2rhosigma2 = t7 * t484 + t223;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t490 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t207 + t451));
            let tv2rhosigma3 = t7 * t490 + t211;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t498 = t187 * v_sigma2;
            let t505 = t88 * v_sigma2;
            let t506 = t430 * t178;
            let t508 = f64x8::splat(1.0) / t80 / t506;
            let t513 = -f64x8::splat(18.92422711111111) * t37 * t180 * t86 + f64x8::splat(157.55207579462962) * t51 * t498 * t86 - f64x8::splat(0.09873882666666667) * t51 * t498 * t96 + f64x8::splat(0.004471634612666667) * t279 * t505 * t508 * t96;
            let t518 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t174 * t219 + t482 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t513));
            let tv2rhosigma5 = t7 * t518 + t223;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t526 = t281 * t38;
            let t528 = f64x8::splat(1.0) / t40 / t526;
            let t533 = -f64x8::splat(59.08202842298611) * t51 * t56 * t46 + f64x8::splat(0.01851353) * t51 * t56 * t60 - f64x8::splat(0.00167686297975) * t279 * t52 * t528 * t60;
            let t537 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t533));
            let tv2sigma20 = t7 * t537;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t544 = t430 * t78;
            let t546 = f64x8::splat(1.0) / t80 / t544;
            let t551 = -f64x8::splat(59.08202842298611) * t51 * t92 * t86 + f64x8::splat(0.01851353) * t51 * t92 * t96 - f64x8::splat(0.00167686297975) * t279 * t88 * t546 * t96;
            let t555 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t551));
            let tv2sigma25 = t7 * t555;
            acc_v2sigma2_5 = tv2sigma25;
            let t559 = f64x8::splat(1.0) / t25 / t20;
            let t560 = t227 * t107;
            let t563 = t226 * t107;
            let t566 = t103 * t103;
            let t567 = f64x8::splat(1.0) / t566;
            let t568 = t17 * t567;
            let t571 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(6.0) * t231 - f64x8::splat(6.0) * t568)));
            let t575 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t559 * t560 + f64x8::splat(10.0) / f64x8::splat(3.0) * t563 * t235 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t571));
            let t576 = t575 * t30;
            let t580 = t239 * t115;
            let t582 = t6 * t580 * t62;
            let t587 = t110 * t252;
            let t589 = t6 * t587 * t62;
            let t592 = t6 * t244 * t134;
            let t598 = f64x8::splat(1.0) / t29 / t103;
            let t599 = t28 * t598;
            let t602 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t599 * t62;
            let t604 = t6 * t253 * t134;
            let t607 = t6 * t116 * t289;
            let t610 = f64x8::splat(1.0) / t40 / t54;
            let t616 = f64x8::splat(1.0) / t39 / t281;
            let t617 = t52 * t616;
            let t621 = f64x8::splat(1.0) / t463;
            let t630 = f64x8::splat(1.0) / t40 / t281 / t54;
            let t635 = t280 * t52;
            let t636 = t281 * t281;
            let t638 = f64x8::splat(1.0) / t636 / t120;
            let t642 = -f64x8::splat(323.81455279012346) * t37 * v_sigma0 * t610 * t46 + f64x8::splat(4621.527556642469) * t51 * t617 * t46 - f64x8::splat(574.5387586925395) * t462 * t621 * t46 - f64x8::splat(2.292934974814815) * t51 * t617 * t60 + f64x8::splat(0.2265628203751111) * t279 * t280 * t630 * t60 - f64x8::splat(3.0353804654393177e-07) * t635 * t638 * t60;
            let t647 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t576 * t62 + f64x8::splat(3.0) / f64x8::splat(10.0) * t582 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t240 * t134 - t589 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(5.0) * t592 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t111 * t289 + t602 - t604 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t607 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t642));
            let t649 = f64x8::splat(1.0) / t73 / t71;
            let t650 = t296 * t142;
            let t653 = t295 * t142;
            let t656 = t68 * t567;
            let t659 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t231 - f64x8::splat(6.0) * t656)));
            let t663 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t649 * t650 + f64x8::splat(10.0) / f64x8::splat(3.0) * t653 * t302 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t659));
            let t664 = t663 * t30;
            let t668 = t306 * t115;
            let t670 = t6 * t668 * t98;
            let t672 = t145 * t252;
            let t674 = t6 * t672 * t98;
            let t676 = t76 * t598;
            let t679 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t676 * t98;
            let t681 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t664 * t98 + f64x8::splat(3.0) / f64x8::splat(10.0) * t670 - t674 / f64x8::splat(10.0) + t679));
            let tv3rho30 = f64x8::splat(3.0) * t294 + f64x8::splat(3.0) * t320 + t7 * (t647 + t681);
            acc_v3rho3_0 = tv3rho30;
            let t684 = f64x8::splat(2.0) * t346;
            let t685 = f64x8::splat(2.0) * t372;
            let t686 = t559 * t159;
            let t689 = t226 * t327;
            let t694 = f64x8::splat(2.0) * t231;
            let t695 = f64x8::splat(6.0) * t568;
            let t697 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t694 - t695)));
            let t701 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t686 * t227 + f64x8::splat(20.0) / f64x8::splat(9.0) * t689 * t107 + f64x8::splat(10.0) / f64x8::splat(9.0) * t323 * t235 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t697));
            let t702 = t701 * t30;
            let t706 = t331 * t115;
            let t709 = t6 * t706 * t62 / f64x8::splat(5.0);
            let t713 = t162 * t252;
            let t715 = t6 * t713 * t62;
            let t719 = t6 * t336 * t134 / f64x8::splat(5.0);
            let t728 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t702 * t62 + t709 + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t332 * t134 - t715 / f64x8::splat(30.0) + t719 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t289 + t582 / f64x8::splat(10.0) - t589 / f64x8::splat(15.0) + t592 / f64x8::splat(5.0) + t602 - t604 / f64x8::splat(15.0) + t607 / f64x8::splat(10.0);
            let t729 = ((t1).select(f64x8::splat(0.0), t728));
            let t730 = t649 * t170;
            let t733 = t295 * t351;
            let t738 = f64x8::splat(6.0) * t656;
            let t740 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t694 - t738)));
            let t744 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t730 * t296 + f64x8::splat(20.0) / f64x8::splat(9.0) * t733 * t142 + f64x8::splat(10.0) / f64x8::splat(9.0) * t347 * t302 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t740));
            let t745 = t744 * t30;
            let t749 = t355 * t115;
            let t752 = t6 * t749 * t98 / f64x8::splat(5.0);
            let t753 = t173 * t252;
            let t755 = t6 * t753 * t98;
            let t764 = t6 * t311 * t192 / f64x8::splat(5.0);
            let t766 = t6 * t315 * t192;
            let t769 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t745 * t98 + t752 - t755 / f64x8::splat(30.0) + t670 / f64x8::splat(10.0) - t674 / f64x8::splat(15.0) + t679 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t307 * t192 + t764 - t766 / f64x8::splat(30.0)));
            let tv3rho31 = t294 + t320 + t684 + t685 + t7 * (t729 + t769);
            acc_v3rho3_1 = tv3rho31;
            let t772 = t559 * t377;
            let t777 = t226 * t382;
            let t781 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t694 - t695)));
            let t785 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t772 * t107 + f64x8::splat(20.0) / f64x8::splat(9.0) * t323 * t327 + f64x8::splat(10.0) / f64x8::splat(9.0) * t777 * t107 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t781));
            let t786 = t785 * t30;
            let t790 = t386 * t115;
            let t792 = t6 * t790 * t62;
            let t801 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t786 * t62 + t792 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t387 * t134 + t709 - t715 / f64x8::splat(15.0) + t719 - t589 / f64x8::splat(30.0) + t602 - t604 / f64x8::splat(30.0)));
            let t802 = t649 * t394;
            let t807 = t295 * t399;
            let t811 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t694 - t738)));
            let t815 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t802 * t142 + f64x8::splat(20.0) / f64x8::splat(9.0) * t347 * t351 + f64x8::splat(10.0) / f64x8::splat(9.0) * t807 * t142 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t811));
            let t816 = t815 * t30;
            let t820 = t403 * t115;
            let t822 = t6 * t820 * t98;
            let t829 = t6 * t360 * t192;
            let t837 = t6 * t150 * t438;
            let t839 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t816 * t98 + t822 / f64x8::splat(10.0) + t752 - t755 / f64x8::splat(15.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t356 * t192 + t829 / f64x8::splat(5.0) - t674 / f64x8::splat(30.0) + t679 + t764 - t766 / f64x8::splat(15.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t438 + t837 / f64x8::splat(10.0);
            let t840 = ((t67).select(f64x8::splat(0.0), t839));
            let tv3rho32 = t684 + t685 + t393 + t443 + t7 * (t801 + t840);
            acc_v3rho3_2 = tv3rho32;
            let t845 = t377 * t159;
            let t852 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t231 - f64x8::splat(6.0) * t568)));
            let t856 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t559 * t845 + f64x8::splat(10.0) / f64x8::splat(3.0) * t323 * t382 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t852));
            let t857 = t856 * t30;
            let t864 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t857 * t62 + f64x8::splat(3.0) / f64x8::splat(10.0) * t792 - t715 / f64x8::splat(10.0) + t602));
            let t865 = t394 * t170;
            let t872 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(6.0) * t231 - f64x8::splat(6.0) * t656)));
            let t876 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t649 * t865 + f64x8::splat(10.0) / f64x8::splat(3.0) * t347 * t399 + f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t872));
            let t877 = t876 * t30;
            let t893 = f64x8::splat(1.0) / t80 / t90;
            let t899 = f64x8::splat(1.0) / t79 / t430;
            let t900 = t88 * t899;
            let t904 = f64x8::splat(1.0) / t506;
            let t913 = f64x8::splat(1.0) / t80 / t430 / t90;
            let t918 = t429 * t88;
            let t919 = t430 * t430;
            let t921 = f64x8::splat(1.0) / t919 / t178;
            let t925 = -f64x8::splat(323.81455279012346) * t37 * v_sigma2 * t893 * t86 + f64x8::splat(4621.527556642469) * t51 * t900 * t86 - f64x8::splat(574.5387586925395) * t505 * t904 * t86 - f64x8::splat(2.292934974814815) * t51 * t900 * t96 + f64x8::splat(0.2265628203751111) * t279 * t429 * t913 * t96 - f64x8::splat(3.0353804654393177e-07) * t918 * t921 * t96;
            let t930 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t877 * t98 + f64x8::splat(3.0) / f64x8::splat(10.0) * t822 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t404 * t192 - t755 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(5.0) * t829 + f64x8::splat(9.0) / f64x8::splat(20.0) * t6 * t174 * t438 + t679 - t766 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t837 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t925));
            let tv3rho33 = f64x8::splat(3.0) * t393 + f64x8::splat(3.0) * t443 + t7 * (t864 + t930);
            acc_v3rho3_3 = tv3rho33;
            let t938 = t6 * t244 * t207;
            let t945 = t6 * t253 * t207 / f64x8::splat(30.0);
            let t947 = t6 * t116 * t470;
            let t952 = t268 * v_sigma0;
            let t956 = f64x8::splat(1.0) / t526;
            let t967 = t280 * v_sigma0;
            let t969 = f64x8::splat(1.0) / t636 / t38;
            let t973 = f64x8::splat(69.38883274074074) * t37 * t261 * t46 - f64x8::splat(1417.9686821516666) * t51 * t952 * t46 + f64x8::splat(215.45203450970234) * t956 * t52 * t46 + f64x8::splat(0.6253459022222222) * t51 * t952 * t60 - f64x8::splat(0.07601778841533334) * t279 * t462 * t284 * t60 + f64x8::splat(1.1382676745397442e-07) * t967 * t969 * t60;
            let t978 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t240 * t207 + t938 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t111 * t470 - t945 + t947 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t973));
            let tv3rho2sigma0 = t7 * t978 + f64x8::splat(2.0) * t475;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t985 = t6 * t311 * t219;
            let t989 = t6 * t315 * t219 / f64x8::splat(30.0);
            let t991 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t307 * t219 + t985 / f64x8::splat(5.0) - t989));
            let tv3rho2sigma2 = t7 * t991 + f64x8::splat(2.0) * t484;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t997 = t6 * t336 * t207;
            let t1005 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t332 * t207 + t997 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t470 + t938 / f64x8::splat(10.0) - t945 + t947 / f64x8::splat(10.0)));
            let tv3rho2sigma3 = t7 * t1005 + t475 + t490;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1011 = t6 * t360 * t219;
            let t1018 = t6 * t150 * t513;
            let t1021 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t356 * t219 + t1011 / f64x8::splat(10.0) + t985 / f64x8::splat(10.0) - t989 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t513 + t1018 / f64x8::splat(10.0)));
            let tv3rho2sigma5 = t7 * t1021 + t484 + t518;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1029 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t387 * t207 + t997 / f64x8::splat(5.0) - t945));
            let tv3rho2sigma6 = t7 * t1029 + f64x8::splat(2.0) * t490;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1043 = t421 * v_sigma2;
            let t1047 = f64x8::splat(1.0) / t544;
            let t1058 = t429 * v_sigma2;
            let t1060 = f64x8::splat(1.0) / t919 / t78;
            let t1064 = f64x8::splat(69.38883274074074) * t37 * t414 * t86 - f64x8::splat(1417.9686821516666) * t51 * t1043 * t86 + f64x8::splat(215.45203450970234) * t1047 * t88 * t86 + f64x8::splat(0.6253459022222222) * t51 * t1043 * t96 - f64x8::splat(0.07601778841533334) * t279 * t505 * t433 * t96 + f64x8::splat(1.1382676745397442e-07) * t1058 * t1060 * t96;
            let t1069 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t404 * t219 + t1011 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t6 * t174 * t513 - t989 + t1018 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t1064));
            let tv3rho2sigma8 = t7 * t1069 + f64x8::splat(2.0) * t518;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1076 = t6 * t116 * t533 / f64x8::splat(10.0);
            let t1080 = t281 * v_rho0;
            let t1081 = f64x8::splat(1.0) / t1080;
            let t1093 = f64x8::splat(1.0) / t636 / v_rho0;
            let t1097 = f64x8::splat(315.10415158925923) * t51 * t129 * t46 - f64x8::splat(80.79451294113838) * t1081 * v_sigma0 * t46 - f64x8::splat(0.09873882666666667) * t51 * t129 * t60 + f64x8::splat(0.022358173063333334) * t279 * t465 * t52 * t60 - f64x8::splat(4.2685037795240406e-08) * t280 * t1093 * t60;
            let t1102 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t111 * t533 + t1076 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t1097));
            let tv3rhosigma20 = t7 * t1102 + t537;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1109 = t6 * t150 * t551 / f64x8::splat(10.0);
            let t1111 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t551 + t1109));
            let tv3rhosigma25 = t7 * t1111 + t555;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1117 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t533 + t1076));
            let tv3rhosigma26 = t7 * t1117 + t537;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1125 = t430 * v_rho1;
            let t1126 = f64x8::splat(1.0) / t1125;
            let t1138 = f64x8::splat(1.0) / t919 / v_rho1;
            let t1142 = f64x8::splat(315.10415158925923) * t51 * t187 * t86 - f64x8::splat(80.79451294113838) * t1126 * v_sigma2 * t86 - f64x8::splat(0.09873882666666667) * t51 * t187 * t96 + f64x8::splat(0.022358173063333334) * t279 * t508 * t88 * t96 - f64x8::splat(4.2685037795240406e-08) * t429 * t1138 * t96;
            let t1147 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t174 * t551 + t1109 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t1142));
            let tv3rhosigma211 = t7 * t1147 + t555;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1156 = f64x8::splat(1.0) / t636;
            let t1160 = f64x8::splat(30.297942352926892) / t281 * t46 - f64x8::splat(0.00503058893925) * t279 * t528 * v_sigma0 * t60 + f64x8::splat(1.6006889173215154e-08) * t462 * t1156 * t60;
            let t1164 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t1160));
            let tv3sigma30 = t7 * t1164;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1172 = f64x8::splat(1.0) / t919;
            let t1176 = f64x8::splat(30.297942352926892) / t430 * t86 - f64x8::splat(0.00503058893925) * t279 * t546 * v_sigma2 * t96 + f64x8::splat(1.6006889173215154e-08) * t505 * t1172 * t96;
            let t1180 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t1176));
            let tv3sigma39 = t7 * t1180;
            acc_v3sigma3_9 = tv3sigma39;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
