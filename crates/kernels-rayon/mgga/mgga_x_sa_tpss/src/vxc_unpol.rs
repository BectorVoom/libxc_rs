//! MGGA_X_SA_TPSS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_sa_tpss.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_sa_tpss_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = ((f64x8::splat(5.0)).sqrt());
            let t22 = f64x8::splat(M_PI) * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = t23 * t23;
            let t25 = v_tau * t24;
            let t26 = t19 * t19;
            let t28 = f64x8::splat(1.0) / t26 / v_rho;
            let t30 = v_sigma * t24;
            let t31 = v_rho * v_rho;
            let t33 = f64x8::splat(1.0) / t26 / t31;
            let t34 = t30 * t33;
            let t36 = t25 * t28 - t34 / f64x8::splat(8.0);
            let t37 = f64x8::splat(M_CBRT6);
            let t38 = t36 * t37;
            let t39 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t40 = (simd::cbrt(t39));
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t38 * t42;
            let t45 = f64x8::splat(5.0) * t43 + f64x8::splat(9.0);
            let t46 = ((t45).sqrt());
            let t47 = f64x8::splat(5.0) / f64x8::splat(9.0) * t43;
            let t48 = t47 + f64x8::splat(0.348);
            let t49 = (simd::ln(t48));
            let t50 = f64x8::splat(2.413) + t49;
            let t51 = ((t50).sqrt());
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t46 * t52;
            let t54 = t22 * t53;
            let t56 = v_sigma * v_sigma;
            let t57 = f64x8::splat(1.0) / t31;
            let t58 = t56 * t57;
            let t59 = v_tau * v_tau;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t58 * t60;
            let t63 = f64x8::splat(1.0) + t61 / f64x8::splat(64.0);
            let t64 = t63 * t63;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t60 * t65;
            let t70 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.02485875) * t58 * t66) * t37;
            let t71 = t70 * t42;
            let t74 = t47 - f64x8::splat(1.0);
            let t75 = t42 * t74;
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t38 * t75;
            let t79 = ((t78).sqrt());
            let t80 = f64x8::splat(1.0) / t79;
            let t83 = t37 * t42;
            let t84 = t83 * t34;
            let t86 = f64x8::splat(9.0) / f64x8::splat(20.0) * t74 * t80 + t84 / f64x8::splat(36.0);
            let t87 = t86 * t86;
            let t90 = t37 * t37;
            let t92 = f64x8::splat(1.0) / t40 / t39;
            let t93 = t90 * t92;
            let t94 = t56 * t23;
            let t95 = t31 * t31;
            let t96 = t95 * v_rho;
            let t98 = f64x8::splat(1.0) / t19 / t96;
            let t100 = t93 * t94 * t98;
            let t102 = f64x8::splat(162.0) * t61 + f64x8::splat(100.0) * t100;
            let t103 = ((t102).sqrt());
            let t108 = f64x8::splat(1.0) / t46;
            let t110 = f64x8::splat(1.0) / f64x8::splat(M_PI) * t21 * t108 * t51;
            let t114 = t56 * v_sigma;
            let t115 = t95 * t95;
            let t116 = f64x8::splat(1.0) / t115;
            let t119 = t71 * t34 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t87 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t86 * t103 + f64x8::splat(25.0) / f64x8::splat(104976.0) * t110 * t100 + f64x8::splat(0.0017218861679299947) * t61 + f64x8::splat(6.013207674276893e-06) * t114 * t116;
            let t121 = f64x8::splat(1.0) + f64x8::splat(0.05165658503789984) * t84;
            let t122 = t121 * t121;
            let t123 = f64x8::splat(1.0) / t122;
            let t125 = f64x8::splat(2.0) / f64x8::splat(45.0) * t54 + t119 * t123;
            let t126 = f64x8::splat(1.0) / t125;
            let t130 = f64x8::splat(1.0) - f64x8::splat(2.0) / f64x8::splat(45.0) * t22 * t53 * t126;
            let t134 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t22 * t53 * t130;
            let t138 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t134));
            let tzk0 = f64x8::splat(2.0) * t138;
            acc_zk = tzk0;
            let t140 = t18 / t26;
            let t144 = t108 * t52;
            let t145 = t22 * t144;
            let t148 = t31 * v_rho;
            let t150 = f64x8::splat(1.0) / t26 / t148;
            let t151 = t30 * t150;
            let t153 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t33 + t151 / f64x8::splat(3.0);
            let t154 = t130 * t153;
            let t159 = f64x8::splat(1.0) / t51 / t50;
            let t160 = t46 * t159;
            let t161 = t22 * t160;
            let t162 = f64x8::splat(1.0) / t48;
            let t163 = t83 * t162;
            let t167 = t126 * t153;
            let t174 = t22 * t46;
            let t175 = t125 * t125;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t52 * t176;
            let t178 = t22 * t108;
            let t183 = t153 * t37;
            let t184 = t42 * t162;
            let t188 = f64x8::splat(1.0) / t148;
            let t189 = t56 * t188;
            let t192 = t56 * t56;
            let t193 = f64x8::splat(1.0) / t96;
            let t194 = t192 * t193;
            let t195 = t59 * t59;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = f64x8::splat(1.0) / t64 / t63;
            let t199 = t196 * t198;
            let t203 = (-f64x8::splat(0.0497175) * t189 * t66 + f64x8::splat(0.001553671875) * t194 * t199) * t37;
            let t204 = t203 * t42;
            let t209 = t42 * t80;
            let t213 = f64x8::splat(1.0) / t79 / t78;
            let t214 = t74 * t213;
            let t217 = t36 * t90;
            let t218 = t92 * t153;
            let t221 = f64x8::splat(0.2222222222222222) * t183 * t75 + f64x8::splat(0.12345679012345678) * t217 * t218;
            let t226 = t183 * t209 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t214 * t221 - f64x8::splat(2.0) / f64x8::splat(27.0) * t83 * t151;
            let t231 = f64x8::splat(1.0) / t103;
            let t232 = t86 * t231;
            let t233 = t189 * t60;
            let t235 = t95 * t31;
            let t237 = f64x8::splat(1.0) / t19 / t235;
            let t239 = t93 * t94 * t237;
            let t241 = -f64x8::splat(324.0) * t233 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t239;
            let t244 = t39 * t39;
            let t247 = f64x8::splat(1.0) / t244 / f64x8::splat(M_PI) * t21;
            let t249 = f64x8::splat(1.0) / t46 / t45;
            let t250 = t249 * t51;
            let t251 = t247 * t250;
            let t252 = t98 * t153;
            let t256 = t247 * t144;
            let t257 = t252 * t162;
            let t264 = t115 * v_rho;
            let t265 = f64x8::splat(1.0) / t264;
            let t268 = t204 * t34 / f64x8::splat(24.0) - t71 * t151 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t86 * t226 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t226 * t103 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t232 * t241 - f64x8::splat(125.0) / f64x8::splat(34992.0) * t251 * t94 * t252 + f64x8::splat(125.0) / f64x8::splat(314928.0) * t256 * t94 * t257 - f64x8::splat(25.0) / f64x8::splat(19683.0) * t110 * t239 - f64x8::splat(0.0034437723358599895) * t233 - f64x8::splat(4.810566139421515e-05) * t114 * t265;
            let t271 = f64x8::splat(1.0) / t122 / t121;
            let t272 = t119 * t271;
            let t273 = t272 * t37;
            let t274 = t42 * v_sigma;
            let t275 = t24 * t150;
            let t276 = t274 * t275;
            let t279 = t178 * t52 * t153 * t83 / f64x8::splat(9.0) - t161 * t183 * t184 / f64x8::splat(81.0) + t268 * t123 + f64x8::splat(0.27550178686879917) * t273 * t276;
            let t283 = -t145 * t167 * t83 / f64x8::splat(9.0) + t161 * t167 * t163 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t174 * t177 * t279;
            let t287 = t145 * t154 * t83 / f64x8::splat(9.0) - t161 * t154 * t163 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t22 * t53 * t283;
            let t292 = ((t3).select(f64x8::splat(0.0), -t7 * t140 * t134 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t287));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t292 + f64x8::splat(2.0) * t138;
            acc_vrho = tvrho0;
            let t295 = t130 * t24;
            let t297 = t33 * t37 * t42;
            let t299 = t145 * t295 * t297;
            let t301 = t160 * t130;
            let t302 = t22 * t301;
            let t303 = t24 * t33;
            let t304 = t303 * t163;
            let t305 = t302 * t304;
            let t307 = t126 * t24;
            let t309 = t145 * t307 * t297;
            let t311 = t160 * t126;
            let t312 = t22 * t311;
            let t313 = t312 * t304;
            let t315 = t303 * t83;
            let t316 = t145 * t315;
            let t318 = t161 * t304;
            let t320 = v_sigma * t57;
            let t323 = f64x8::splat(1.0) / t95;
            let t324 = t114 * t323;
            let t328 = (f64x8::splat(0.0497175) * t320 * t66 - f64x8::splat(0.001553671875) * t324 * t199) * t37;
            let t329 = t328 * t42;
            let t332 = t42 * t24;
            let t333 = t332 * t33;
            let t336 = t83 * t80;
            let t337 = t303 * t336;
            let t339 = t83 * t74;
            let t340 = t303 * t339;
            let t342 = t92 * t24;
            let t344 = t217 * t342 * t33;
            let t346 = -f64x8::splat(0.027777777777777776) * t340 - f64x8::splat(0.015432098765432098) * t344;
            let t350 = -t337 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t214 * t346 + t315 / f64x8::splat(36.0);
            let t355 = t320 * t60;
            let t357 = v_sigma * t23;
            let t359 = t93 * t357 * t98;
            let t361 = f64x8::splat(324.0) * t355 + f64x8::splat(200.0) * t359;
            let t364 = t247 * t249;
            let t365 = t51 * t56;
            let t367 = t364 * t365 * t116;
            let t369 = t247 * t108;
            let t370 = t52 * t56;
            let t371 = t116 * t162;
            let t373 = t369 * t370 * t371;
            let t378 = t56 * t116;
            let t380 = t329 * t34 / f64x8::splat(24.0) + t70 * t333 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t86 * t350 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t350 * t103 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t232 * t361 + f64x8::splat(125.0) / f64x8::splat(139968.0) * t367 - f64x8::splat(125.0) / f64x8::splat(1259712.0) * t373 + f64x8::splat(25.0) / f64x8::splat(52488.0) * t110 * t359 + f64x8::splat(0.0034437723358599895) * t355 + f64x8::splat(1.803962302283068e-05) * t378;
            let t382 = t272 * t24;
            let t385 = -t316 / f64x8::splat(72.0) + t318 / f64x8::splat(648.0) + t380 * t123 - f64x8::splat(0.10331317007579968) * t382 * t297;
            let t389 = t309 / f64x8::splat(72.0) - t313 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t174 * t177 * t385;
            let t393 = -t299 / f64x8::splat(72.0) + t305 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t22 * t53 * t389;
            let t397 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t393));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t397;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t400 = t28 * t37 * t42;
            let t404 = t24 * t28;
            let t405 = t404 * t163;
            let t413 = t404 * t83;
            let t418 = t59 * v_tau;
            let t419 = f64x8::splat(1.0) / t418;
            let t420 = t419 * t65;
            let t423 = t192 * t323;
            let t424 = t195 * v_tau;
            let t425 = f64x8::splat(1.0) / t424;
            let t426 = t425 * t198;
            let t430 = (-f64x8::splat(0.0497175) * t58 * t420 + f64x8::splat(0.001553671875) * t423 * t426) * t37;
            let t431 = t430 * t42;
            let t441 = f64x8::splat(0.2222222222222222) * t404 * t339 + f64x8::splat(0.12345679012345678) * t217 * t342 * t28;
            let t444 = t404 * t336 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t214 * t441;
            let t449 = t58 * t419;
            let t452 = t95 * t148;
            let t453 = f64x8::splat(1.0) / t452;
            let t457 = t453 * t162;
            let t462 = t431 * t34 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t86 * t444 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t444 * t103 + f64x8::splat(73.0) / f64x8::splat(600.0) * t232 * t449 - f64x8::splat(125.0) / f64x8::splat(17496.0) * t364 * t365 * t453 + f64x8::splat(125.0) / f64x8::splat(157464.0) * t369 * t370 * t457 - f64x8::splat(0.0034437723358599895) * t449;
            let t464 = t145 * t413 / f64x8::splat(9.0) - t161 * t405 / f64x8::splat(81.0) + t462 * t123;
            let t468 = -t145 * t307 * t400 / f64x8::splat(9.0) + t312 * t405 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t174 * t177 * t464;
            let t472 = t145 * t295 * t400 / f64x8::splat(9.0) - t302 * t405 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t22 * t53 * t468;
            let t476 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t472));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t476;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
