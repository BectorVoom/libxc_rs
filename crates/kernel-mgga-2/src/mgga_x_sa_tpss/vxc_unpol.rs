//! MGGA_X_SA_TPSS vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_sa_tpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_sa_tpss_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = f64::sqrt(5.0);
        let t22 = M_PI * t21;
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = tau[ip] * t24;
        let t26 = t19 * t19;
        let t28 = 1.0 / t26 / rho[ip];
        let t30 = sigma[ip] * t24;
        let t31 = rho[ip] * rho[ip];
        let t33 = 1.0 / t26 / t31;
        let t34 = t30 * t33;
        let t36 = t25 * t28 - t34 / 8.0;
        let t37 = M_CBRT6;
        let t38 = t36 * t37;
        let t39 = M_PI * M_PI;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t38 * t42;
        let t45 = 5.0 * t43 + 9.0;
        let t46 = f64::sqrt(t45);
        let t47 = 5.0 / 9.0 * t43;
        let t48 = t47 + 0.348e0;
        let t49 = f64::ln(t48);
        let t50 = 0.2413e1 + t49;
        let t51 = f64::sqrt(t50);
        let t52 = 1.0 / t51;
        let t53 = t46 * t52;
        let t54 = t22 * t53;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = 1.0 / t31;
        let t58 = t56 * t57;
        let t59 = tau[ip] * tau[ip];
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t63 = 1.0 + t61 / 64.0;
        let t64 = t63 * t63;
        let t65 = 1.0 / t64;
        let t66 = t60 * t65;
        let t70 = (10.0 / 81.0 + 0.2485875e-1 * t58 * t66) * t37;
        let t71 = t70 * t42;
        let t74 = t47 - 1.0;
        let t75 = t42 * t74;
        let t78 = 1.0 + 0.22222222222222222222e0 * t38 * t75;
        let t79 = f64::sqrt(t78);
        let t80 = 1.0 / t79;
        let t83 = t37 * t42;
        let t84 = t83 * t34;
        let t86 = 9.0 / 20.0 * t74 * t80 + t84 / 36.0;
        let t87 = t86 * t86;
        let t90 = t37 * t37;
        let t92 = 1.0 / t40 / t39;
        let t93 = t90 * t92;
        let t94 = t56 * t23;
        let t95 = t31 * t31;
        let t96 = t95 * rho[ip];
        let t98 = 1.0 / t19 / t96;
        let t100 = t93 * t94 * t98;
        let t102 = 162.0 * t61 + 100.0 * t100;
        let t103 = f64::sqrt(t102);
        let t108 = 1.0 / t46;
        let t110 = 1.0 / M_PI * t21 * t108 * t51;
        let t114 = t56 * sigma[ip];
        let t115 = t95 * t95;
        let t116 = 1.0 / t115;
        let t119 = t71 * t34 / 24.0 + 146.0 / 2025.0 * t87 - 73.0 / 97200.0 * t86 * t103 + 25.0 / 104976.0 * t110 * t100 + 0.17218861679299947194e-2 * t61 + 0.60132076742768935544e-5 * t114 * t116;
        let t121 = 1.0 + 0.51656585037899841583e-1 * t84;
        let t122 = t121 * t121;
        let t123 = 1.0 / t122;
        let t125 = 2.0 / 45.0 * t54 + t119 * t123;
        let t126 = 1.0 / t125;
        let t130 = 1.0 - 2.0 / 45.0 * t22 * t53 * t126;
        let t134 = 1.0 + 2.0 / 45.0 * t22 * t53 * t130;
        let t138 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t134);
        let tzk0 = 2.0 * t138;
        zk[ip] += tzk0;
        let t140 = t18 / t26;
        let t144 = t108 * t52;
        let t145 = t22 * t144;
        let t148 = t31 * rho[ip];
        let t150 = 1.0 / t26 / t148;
        let t151 = t30 * t150;
        let t153 = -5.0 / 3.0 * t25 * t33 + t151 / 3.0;
        let t154 = t130 * t153;
        let t159 = 1.0 / t51 / t50;
        let t160 = t46 * t159;
        let t161 = t22 * t160;
        let t162 = 1.0 / t48;
        let t163 = t83 * t162;
        let t167 = t126 * t153;
        let t174 = t22 * t46;
        let t175 = t125 * t125;
        let t176 = 1.0 / t175;
        let t177 = t52 * t176;
        let t178 = t22 * t108;
        let t183 = t153 * t37;
        let t184 = t42 * t162;
        let t188 = 1.0 / t148;
        let t189 = t56 * t188;
        let t192 = t56 * t56;
        let t193 = 1.0 / t96;
        let t194 = t192 * t193;
        let t195 = t59 * t59;
        let t196 = 1.0 / t195;
        let t198 = 1.0 / t64 / t63;
        let t199 = t196 * t198;
        let t203 = (-0.497175e-1 * t189 * t66 + 0.1553671875e-2 * t194 * t199) * t37;
        let t204 = t203 * t42;
        let t209 = t42 * t80;
        let t213 = 1.0 / t79 / t78;
        let t214 = t74 * t213;
        let t217 = t36 * t90;
        let t218 = t92 * t153;
        let t221 = 0.22222222222222222222e0 * t183 * t75 + 0.12345679012345679012e0 * t217 * t218;
        let t226 = t183 * t209 / 4.0 - 9.0 / 40.0 * t214 * t221 - 2.0 / 27.0 * t83 * t151;
        let t231 = 1.0 / t103;
        let t232 = t86 * t231;
        let t233 = t189 * t60;
        let t235 = t95 * t31;
        let t237 = 1.0 / t19 / t235;
        let t239 = t93 * t94 * t237;
        let t241 = -324.0 * t233 - 1600.0 / 3.0 * t239;
        let t244 = t39 * t39;
        let t247 = 1.0 / t244 / M_PI * t21;
        let t249 = 1.0 / t46 / t45;
        let t250 = t249 * t51;
        let t251 = t247 * t250;
        let t252 = t98 * t153;
        let t256 = t247 * t144;
        let t257 = t252 * t162;
        let t264 = t115 * rho[ip];
        let t265 = 1.0 / t264;
        let t268 = t204 * t34 / 24.0 - t71 * t151 / 9.0 + 292.0 / 2025.0 * t86 * t226 - 73.0 / 97200.0 * t226 * t103 - 73.0 / 194400.0 * t232 * t241 - 125.0 / 34992.0 * t251 * t94 * t252 + 125.0 / 314928.0 * t256 * t94 * t257 - 25.0 / 19683.0 * t110 * t239 - 0.34437723358599894388e-2 * t233 - 0.48105661394215148435e-4 * t114 * t265;
        let t271 = 1.0 / t122 / t121;
        let t272 = t119 * t271;
        let t273 = t272 * t37;
        let t274 = t42 * sigma[ip];
        let t275 = t24 * t150;
        let t276 = t274 * t275;
        let t279 = t178 * t52 * t153 * t83 / 9.0 - t161 * t183 * t184 / 81.0 + t268 * t123 + 0.2755017868687991551e0 * t273 * t276;
        let t283 = -t145 * t167 * t83 / 9.0 + t161 * t167 * t163 / 81.0 + 2.0 / 45.0 * t174 * t177 * t279;
        let t287 = t145 * t154 * t83 / 9.0 - t161 * t154 * t163 / 81.0 + 2.0 / 45.0 * t22 * t53 * t283;
        let t292 = piecewise3(t3, 0.0, -t7 * t140 * t134 / 8.0 - 3.0 / 8.0 * t7 * t20 * t287);
        let tvrho0 = 2.0 * rho[ip] * t292 + 2.0 * t138;
        vrho[ip] += tvrho0;
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
        let t320 = sigma[ip] * t57;
        let t323 = 1.0 / t95;
        let t324 = t114 * t323;
        let t328 = (0.497175e-1 * t320 * t66 - 0.1553671875e-2 * t324 * t199) * t37;
        let t329 = t328 * t42;
        let t332 = t42 * t24;
        let t333 = t332 * t33;
        let t336 = t83 * t80;
        let t337 = t303 * t336;
        let t339 = t83 * t74;
        let t340 = t303 * t339;
        let t342 = t92 * t24;
        let t344 = t217 * t342 * t33;
        let t346 = -0.27777777777777777778e-1 * t340 - 0.15432098765432098765e-1 * t344;
        let t350 = -t337 / 32.0 - 9.0 / 40.0 * t214 * t346 + t315 / 36.0;
        let t355 = t320 * t60;
        let t357 = sigma[ip] * t23;
        let t359 = t93 * t357 * t98;
        let t361 = 324.0 * t355 + 200.0 * t359;
        let t364 = t247 * t249;
        let t365 = t51 * t56;
        let t367 = t364 * t365 * t116;
        let t369 = t247 * t108;
        let t370 = t52 * t56;
        let t371 = t116 * t162;
        let t373 = t369 * t370 * t371;
        let t378 = t56 * t116;
        let t380 = t329 * t34 / 24.0 + t70 * t333 / 24.0 + 292.0 / 2025.0 * t86 * t350 - 73.0 / 97200.0 * t350 * t103 - 73.0 / 194400.0 * t232 * t361 + 125.0 / 139968.0 * t367 - 125.0 / 1259712.0 * t373 + 25.0 / 52488.0 * t110 * t359 + 0.34437723358599894388e-2 * t355 + 0.18039623022830680663e-4 * t378;
        let t382 = t272 * t24;
        let t385 = -t316 / 72.0 + t318 / 648.0 + t380 * t123 - 0.10331317007579968317e0 * t382 * t297;
        let t389 = t309 / 72.0 - t313 / 648.0 + 2.0 / 45.0 * t174 * t177 * t385;
        let t393 = -t299 / 72.0 + t305 / 648.0 + 2.0 / 45.0 * t22 * t53 * t389;
        let t397 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t393);
        let tvsigma0 = 2.0 * rho[ip] * t397;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t400 = t28 * t37 * t42;
        let t404 = t24 * t28;
        let t405 = t404 * t163;
        let t413 = t404 * t83;
        let t418 = t59 * tau[ip];
        let t419 = 1.0 / t418;
        let t420 = t419 * t65;
        let t423 = t192 * t323;
        let t424 = t195 * tau[ip];
        let t425 = 1.0 / t424;
        let t426 = t425 * t198;
        let t430 = (-0.497175e-1 * t58 * t420 + 0.1553671875e-2 * t423 * t426) * t37;
        let t431 = t430 * t42;
        let t441 = 0.22222222222222222222e0 * t404 * t339 + 0.12345679012345679012e0 * t217 * t342 * t28;
        let t444 = t404 * t336 / 4.0 - 9.0 / 40.0 * t214 * t441;
        let t449 = t58 * t419;
        let t452 = t95 * t148;
        let t453 = 1.0 / t452;
        let t457 = t453 * t162;
        let t462 = t431 * t34 / 24.0 + 292.0 / 2025.0 * t86 * t444 - 73.0 / 97200.0 * t444 * t103 + 73.0 / 600.0 * t232 * t449 - 125.0 / 17496.0 * t364 * t365 * t453 + 125.0 / 157464.0 * t369 * t370 * t457 - 0.34437723358599894388e-2 * t449;
        let t464 = t145 * t413 / 9.0 - t161 * t405 / 81.0 + t462 * t123;
        let t468 = -t145 * t307 * t400 / 9.0 + t312 * t405 / 81.0 + 2.0 / 45.0 * t174 * t177 * t464;
        let t472 = t145 * t295 * t400 / 9.0 - t302 * t405 / 81.0 + 2.0 / 45.0 * t22 * t53 * t468;
        let t476 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t472);
        let tvtau0 = 2.0 * rho[ip] * t476;
        vtau[ip] += tvtau0;
    }
}
