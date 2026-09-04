//! GGA_X_ITYH_PBE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_pbe.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_pbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t6 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t21 = f64x8::splat(M_PI) * t20;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = param_mu * t27;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t34 = f64x8::splat(M_CBRT2);
            let t35 = t34 * t34;
            let t36 = v_sigma * t35;
            let t37 = v_rho * v_rho;
            let t38 = t19 * t19;
            let t40 = f64x8::splat(1.0) / t38 / t37;
            let t44 = param_kappa + t28 * t32 * t36 * t40 / f64x8::splat(24.0);
            let t49 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t44);
            let t52 = t21 * t26 / t49;
            let t53 = ((t52).sqrt());
            let t55 = param_hyb_omega_0 / t53;
            let t56 = t11 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t55 * t34 * t58 / f64x8::splat(2.0);
            let t62 = (f64x8::splat(1.35)).simd_le(t61);
            let t63 = (f64x8::splat(1.35)).simd_lt(t61);
            let t64 = ((t63).select(t61, f64x8::splat(1.35)));
            let t65 = t64 * t64;
            let t68 = t65 * t65;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = t68 * t65;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t68 * t68;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t74 / t65;
            let t81 = f64x8::splat(1.0) / t74 / t68;
            let t84 = f64x8::splat(1.0) / t74 / t71;
            let t86 = t74 * t74;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = ((t63).select(f64x8::splat(1.35), t61));
            let t91 = ((f64x8::splat(M_PI)).sqrt());
            let t92 = f64x8::splat(1.0) / t90;
            let t94 = (simd::erf(t92 / f64x8::splat(2.0)));
            let t96 = t90 * t90;
            let t97 = f64x8::splat(1.0) / t96;
            let t99 = (simd::exp(-t97 / f64x8::splat(4.0)));
            let t100 = t99 - f64x8::splat(1.0);
            let t103 = t99 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t96 * t100;
            let t106 = f64x8::splat(2.0) * t90 * t103 + t91 * t94;
            let t110 = ((t62).select(f64x8::splat(1.0) / t65 / f64x8::splat(36.0) - t69 / f64x8::splat(960.0) + t72 / f64x8::splat(26880.0) - t75 / f64x8::splat(829440.0) + t78 / f64x8::splat(28385280.0) - t81 / f64x8::splat(1073479680.0) + t84 / f64x8::splat(44590694400.0) - t87 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t106));
            let t115 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t110 * t49));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
            let t116 = f64x8::splat(1.0) / t38;
            let t121 = t65 * t64;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = param_hyb_omega_0 / t53 / t52;
            let t126 = t125 * t58;
            let t127 = t21 * t26;
            let t128 = t126 * t127;
            let t129 = t49 * t49;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = param_kappa * param_kappa;
            let t132 = t130 * t131;
            let t133 = t44 * t44;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t134 * param_mu;
            let t136 = t132 * t135;
            let t137 = t27 * t32;
            let t138 = t37 * v_rho;
            let t140 = f64x8::splat(1.0) / t38 / t138;
            let t143 = t136 * t137 * v_sigma * t140;
            let t147 = f64x8::splat(1.0) / t57 / t56;
            let t152 = -t128 * t143 / f64x8::splat(18.0) - t55 * t34 * t147 * t11 / f64x8::splat(6.0);
            let t153 = ((t63).select(t152, f64x8::splat(0.0)));
            let t156 = t68 * t64;
            let t157 = f64x8::splat(1.0) / t156;
            let t160 = t68 * t121;
            let t161 = f64x8::splat(1.0) / t160;
            let t165 = f64x8::splat(1.0) / t74 / t64;
            let t169 = f64x8::splat(1.0) / t74 / t121;
            let t173 = f64x8::splat(1.0) / t74 / t156;
            let t177 = f64x8::splat(1.0) / t74 / t160;
            let t181 = f64x8::splat(1.0) / t86 / t64;
            let t185 = ((t63).select(f64x8::splat(0.0), t152));
            let t187 = t99 * t97;
            let t191 = t96 * t90;
            let t192 = f64x8::splat(1.0) / t191;
            let t196 = t90 * t100;
            let t201 = t192 * t185 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t196 * t185 - t92 * t185 * t99;
            let t204 = f64x8::splat(2.0) * t185 * t103 - t187 * t185 + f64x8::splat(2.0) * t90 * t201;
            let t208 = ((t62).select(-t122 * t153 / f64x8::splat(18.0) + t157 * t153 / f64x8::splat(240.0) - t161 * t153 / f64x8::splat(4480.0) + t165 * t153 / f64x8::splat(103680.0) - t169 * t153 / f64x8::splat(2838528.0) + t173 * t153 / f64x8::splat(89456640.0) - t177 * t153 / f64x8::splat(3185049600.0) + t181 * t153 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t185 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t204));
            let t214 = f64x8::splat(1.0) / t19 / t138;
            let t219 = t32 * v_sigma;
            let t220 = t219 * t35;
            let t221 = t135 * t27 * t220;
            let t225 = ((t2).select(f64x8::splat(0.0), -t18 * t116 * t110 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t208 * t49 + t18 * t214 * t110 * t131 * t221 / f64x8::splat(24.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t225 + f64x8::splat(2.0) * t115;
            acc_vrho = tvrho0;
            let t228 = t132 * t134;
            let t233 = t128 * t228 * t28 * t32 * t40 / f64x8::splat(48.0);
            let t234 = ((t63).select(t233, f64x8::splat(0.0)));
            let t237 = t157 * t234;
            let t239 = t161 * t234;
            let t241 = t165 * t234;
            let t243 = t169 * t234;
            let t245 = t173 * t234;
            let t247 = t177 * t234;
            let t249 = t181 * t234;
            let t252 = ((t63).select(f64x8::splat(0.0), t233));
            let t264 = t192 * t252 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t196 * t252 - t92 * t252 * t99;
            let t267 = f64x8::splat(2.0) * t252 * t103 - t187 * t252 + f64x8::splat(2.0) * t90 * t264;
            let t271 = ((t62).select(-t122 * t234 / f64x8::splat(18.0) + t237 / f64x8::splat(240.0) - t239 / f64x8::splat(4480.0) + t241 / f64x8::splat(103680.0) - t243 / f64x8::splat(2838528.0) + t245 / f64x8::splat(89456640.0) - t247 / f64x8::splat(3185049600.0) + t249 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t252 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t267));
            let t278 = t17 / t19 / t37;
            let t281 = t131 * t134;
            let t283 = t137 * t35;
            let t284 = t281 * param_mu * t283;
            let t288 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t271 * t49 - t6 * t278 * t110 * t284 / f64x8::splat(64.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t288;
            acc_vsigma = tvsigma0;
            let t292 = f64x8::splat(1.0) / t38 / v_rho;
            let t301 = t37 * t37;
            let t303 = f64x8::splat(1.0) / t19 / t301;
            let t309 = t153 * t153;
            let t312 = t29 * t3;
            let t313 = t23 * t23;
            let t314 = f64x8::splat(1.0) / t313;
            let t315 = t25 * t25;
            let t316 = t314 * t315;
            let t322 = param_hyb_omega_0 / t53 / t312 / t316 / t130 / f64x8::splat(3.0);
            let t326 = t129 * t129;
            let t327 = f64x8::splat(1.0) / t326;
            let t328 = t315 * t327;
            let t329 = t3 * t314 * t328;
            let t330 = t322 * t58 * t29 * t329;
            let t331 = t131 * t131;
            let t332 = t133 * t133;
            let t333 = f64x8::splat(1.0) / t332;
            let t335 = param_mu * param_mu;
            let t336 = t27 * t27;
            let t337 = t335 * t336;
            let t338 = t331 * t333 * t337;
            let t340 = f64x8::splat(1.0) / t30 / t29;
            let t341 = v_sigma * v_sigma;
            let t342 = t340 * t341;
            let t343 = t301 * t138;
            let t345 = f64x8::splat(1.0) / t19 / t343;
            let t346 = t345 * t35;
            let t347 = t342 * t346;
            let t348 = t338 * t347;
            let t352 = t125 * t147 * f64x8::splat(M_PI);
            let t353 = t20 * t24;
            let t354 = t25 * t130;
            let t355 = t353 * t354;
            let t356 = t352 * t355;
            let t357 = t281 * t28;
            let t358 = t140 * t11;
            let t364 = t125 * t58 * f64x8::splat(M_PI);
            let t365 = t129 * t49;
            let t366 = f64x8::splat(1.0) / t365;
            let t367 = t25 * t366;
            let t368 = t353 * t367;
            let t369 = t364 * t368;
            let t372 = t364 * t355;
            let t373 = t133 * t44;
            let t374 = f64x8::splat(1.0) / t373;
            let t375 = t131 * t374;
            let t376 = t375 * t337;
            let t381 = f64x8::splat(1.0) / t38 / t301;
            let t387 = t11 * t11;
            let t390 = f64x8::splat(1.0) / t57 / t387 / t37;
            let t395 = t330 * t348 / f64x8::splat(36.0) + t356 * t357 * t219 * t358 / f64x8::splat(27.0) - t369 * t348 / f64x8::splat(81.0) - t372 * t376 * t347 / f64x8::splat(81.0) + f64x8::splat(11.0) / f64x8::splat(54.0) * t128 * t136 * t137 * v_sigma * t381 + f64x8::splat(2.0) / f64x8::splat(9.0) * t55 * t34 * t390 * t387;
            let t396 = ((t63).select(t395, f64x8::splat(0.0)));
            let t424 = f64x8::splat(1.0) / t86 / t65;
            let t429 = t69 * t309 / f64x8::splat(6.0) - t122 * t396 / f64x8::splat(18.0) - t72 * t309 / f64x8::splat(48.0) + t157 * t396 / f64x8::splat(240.0) + t75 * t309 / f64x8::splat(640.0) - t161 * t396 / f64x8::splat(4480.0) - t78 * t309 / f64x8::splat(11520.0) + t165 * t396 / f64x8::splat(103680.0) + t81 * t309 / f64x8::splat(258048.0) - t169 * t396 / f64x8::splat(2838528.0) - t84 * t309 / f64x8::splat(6881280.0) + t173 * t396 / f64x8::splat(89456640.0) + t87 * t309 / f64x8::splat(212336640.0) - t177 * t396 / f64x8::splat(3185049600.0) - t424 * t309 / f64x8::splat(7431782400.0) + t181 * t396 / f64x8::splat(126340300800.0);
            let t430 = ((t63).select(f64x8::splat(0.0), t395));
            let t435 = t96 * t96;
            let t437 = f64x8::splat(1.0) / t435 / t90;
            let t438 = t185 * t185;
            let t439 = t437 * t438;
            let t442 = t99 * t192;
            let t450 = f64x8::splat(1.0) / t435;
            let t458 = f64x8::splat(1.0) / t435 / t96;
            let t459 = t458 * t438;
            let t470 = -f64x8::splat(2.0) * t450 * t438 * t99 + t192 * t430 * t99 / f64x8::splat(2.0) + t459 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t438 * t100 - t97 * t438 * t99 - f64x8::splat(4.0) * t196 * t430 - t92 * t430 * t99;
            let t473 = -t439 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t442 * t438 - t187 * t430 + f64x8::splat(2.0) * t430 * t103 + f64x8::splat(4.0) * t185 * t201 + f64x8::splat(2.0) * t90 * t470;
            let t477 = ((t62).select(t429, -f64x8::splat(8.0) / f64x8::splat(3.0) * t430 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t185 * t204 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t473));
            let t487 = f64x8::splat(1.0) / t343;
            let t490 = t18 * t487 * t110 * t131;
            let t491 = t374 * t335;
            let t492 = t491 * t336;
            let t494 = t492 * t342 * t34;
            let t498 = ((t2).select(f64x8::splat(0.0), t18 * t292 * t110 * t49 / f64x8::splat(12.0) - t18 * t116 * t208 * t49 / f64x8::splat(4.0) - t18 * t303 * t110 * t131 * t221 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t477 * t49 + t18 * t214 * t208 * t131 * t221 / f64x8::splat(12.0) + t490 * t494 / f64x8::splat(54.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t498 + f64x8::splat(4.0) * t225;
            acc_v2rho2 = tv2rho20;
            let t505 = t69 * t234;
            let t508 = t301 * t37;
            let t510 = f64x8::splat(1.0) / t19 / t508;
            let t511 = t340 * t510;
            let t512 = t511 * t36;
            let t513 = t338 * t512;
            let t516 = t125 * t147;
            let t517 = t516 * t127;
            let t533 = -t330 * t513 / f64x8::splat(96.0) - t517 * t136 * t137 * t40 * t11 / f64x8::splat(144.0) + t369 * t513 / f64x8::splat(216.0) + t372 * t376 * t512 / f64x8::splat(216.0) - t128 * t228 * t28 * t32 * t140 / f64x8::splat(18.0);
            let t534 = ((t63).select(t533, f64x8::splat(0.0)));
            let t537 = t72 * t234;
            let t540 = t157 * t534;
            let t542 = t75 * t234;
            let t545 = t161 * t534;
            let t547 = t78 * t234;
            let t550 = t165 * t534;
            let t552 = t81 * t234;
            let t555 = t169 * t534;
            let t557 = t84 * t234;
            let t560 = t173 * t534;
            let t562 = t87 * t234;
            let t565 = t177 * t534;
            let t567 = t424 * t234;
            let t570 = t181 * t534;
            let t572 = t505 * t153 / f64x8::splat(6.0) - t122 * t534 / f64x8::splat(18.0) - t537 * t153 / f64x8::splat(48.0) + t540 / f64x8::splat(240.0) + t542 * t153 / f64x8::splat(640.0) - t545 / f64x8::splat(4480.0) - t547 * t153 / f64x8::splat(11520.0) + t550 / f64x8::splat(103680.0) + t552 * t153 / f64x8::splat(258048.0) - t555 / f64x8::splat(2838528.0) - t557 * t153 / f64x8::splat(6881280.0) + t560 / f64x8::splat(89456640.0) + t562 * t153 / f64x8::splat(212336640.0) - t565 / f64x8::splat(3185049600.0) - t567 * t153 / f64x8::splat(7431782400.0) + t570 / f64x8::splat(126340300800.0);
            let t573 = ((t63).select(f64x8::splat(0.0), t533));
            let t577 = t437 * t185;
            let t578 = t99 * t252;
            let t581 = t252 * t185;
            let t591 = t450 * t252;
            let t592 = t99 * t185;
            let t598 = t458 * t252;
            let t601 = t185 * t100;
            let t604 = t97 * t185;
            let t610 = -f64x8::splat(2.0) * t591 * t592 + t192 * t573 * t99 / f64x8::splat(2.0) + t598 * t592 / f64x8::splat(4.0) - f64x8::splat(4.0) * t601 * t252 - t604 * t578 - f64x8::splat(4.0) * t196 * t573 - t92 * t573 * t99;
            let t613 = -t577 * t578 / f64x8::splat(2.0) + f64x8::splat(2.0) * t442 * t581 - t187 * t573 + f64x8::splat(2.0) * t573 * t103 + f64x8::splat(2.0) * t252 * t201 + f64x8::splat(2.0) * t185 * t264 + f64x8::splat(2.0) * t90 * t610;
            let t617 = ((t62).select(t572, -f64x8::splat(8.0) / f64x8::splat(3.0) * t573 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t185 * t267 - f64x8::splat(8.0) / f64x8::splat(3.0) * t252 * t204 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t613));
            let t627 = t17 * t214;
            let t636 = f64x8::splat(1.0) / t508;
            let t642 = t492 * t340 * t34 * v_sigma;
            let t646 = ((t2).select(f64x8::splat(0.0), -t18 * t116 * t271 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t617 * t49 + t18 * t214 * t271 * t131 * t221 / f64x8::splat(24.0) + f64x8::splat(7.0) / f64x8::splat(192.0) * t6 * t627 * t110 * t284 - t6 * t278 * t208 * t284 / f64x8::splat(64.0) - t18 * t636 * t110 * t131 * t642 / f64x8::splat(144.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t646 + f64x8::splat(2.0) * t288;
            acc_v2rhosigma = tv2rhosigma0;
            let t649 = t234 * t234;
            let t652 = t322 * t58;
            let t654 = t652 * t312 * t316;
            let t655 = t327 * t331;
            let t656 = t333 * t335;
            let t657 = t655 * t656;
            let t658 = t336 * t340;
            let t659 = t301 * v_rho;
            let t661 = f64x8::splat(1.0) / t19 / t659;
            let t663 = t658 * t661 * t35;
            let t667 = t366 * t331;
            let t668 = t667 * t656;
            let t672 = t132 * t491;
            let t676 = t654 * t657 * t663 / f64x8::splat(256.0) - t128 * t668 * t663 / f64x8::splat(576.0) - t128 * t672 * t663 / f64x8::splat(576.0);
            let t677 = ((t63).select(t676, f64x8::splat(0.0)));
            let t680 = t72 * t649;
            let t682 = t157 * t677;
            let t684 = t75 * t649;
            let t686 = t161 * t677;
            let t688 = t78 * t649;
            let t690 = t165 * t677;
            let t692 = t81 * t649;
            let t694 = t169 * t677;
            let t696 = t84 * t649;
            let t698 = t173 * t677;
            let t700 = t87 * t649;
            let t702 = t177 * t677;
            let t704 = t424 * t649;
            let t706 = t181 * t677;
            let t708 = t69 * t649 / f64x8::splat(6.0) - t122 * t677 / f64x8::splat(18.0) - t680 / f64x8::splat(48.0) + t682 / f64x8::splat(240.0) + t684 / f64x8::splat(640.0) - t686 / f64x8::splat(4480.0) - t688 / f64x8::splat(11520.0) + t690 / f64x8::splat(103680.0) + t692 / f64x8::splat(258048.0) - t694 / f64x8::splat(2838528.0) - t696 / f64x8::splat(6881280.0) + t698 / f64x8::splat(89456640.0) + t700 / f64x8::splat(212336640.0) - t702 / f64x8::splat(3185049600.0) - t704 / f64x8::splat(7431782400.0) + t706 / f64x8::splat(126340300800.0);
            let t709 = ((t63).select(f64x8::splat(0.0), t676));
            let t714 = t252 * t252;
            let t715 = t437 * t714;
            let t731 = t458 * t714;
            let t742 = -f64x8::splat(2.0) * t450 * t714 * t99 + t192 * t709 * t99 / f64x8::splat(2.0) + t731 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t714 * t100 - t97 * t714 * t99 - f64x8::splat(4.0) * t196 * t709 - t92 * t709 * t99;
            let t745 = -t715 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t442 * t714 - t187 * t709 + f64x8::splat(2.0) * t709 * t103 + f64x8::splat(4.0) * t252 * t264 + f64x8::splat(2.0) * t90 * t742;
            let t749 = ((t62).select(t708, -f64x8::splat(8.0) / f64x8::splat(3.0) * t709 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t252 * t267 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t745));
            let t759 = t17 / t659;
            let t764 = t375 * t335 * t658 * t34;
            let t768 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t749 * t49 - t6 * t278 * t271 * t284 / f64x8::splat(32.0) + t6 * t759 * t110 * t764 / f64x8::splat(384.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t768;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
