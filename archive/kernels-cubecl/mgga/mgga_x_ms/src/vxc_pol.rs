//! MGGA_X_MS vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ms.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_ms_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3::<f64>(t20);
        let t26 = piecewise3::<f64>(t21, t23, t24 * t20);
        let t27 = pow_1_3::<f64>(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t42 = 5.0 / 972.0 * t34 * t40;
        let t43 = param_kappa + t42;
        let t47 = param_kappa * (1.0 - param_kappa / t43);
        let t49 = 1.0 / t37 / rho0;
        let t52 = tau0 * t49 - t40 / 8.0;
        let t53 = t52 * t52;
        let t54 = t29 * t29;
        let t57 = 1.0 / t31 / t30;
        let t60 = 1.0 - 25.0 / 81.0 * t53 * t54 * t57;
        let t61 = t60 * t60;
        let t62 = t61 * t60;
        let t63 = t53 * t52;
        let t64 = t30 * t30;
        let t65 = 1.0 / t64;
        let t68 = t53 * t53;
        let t71 = t64 * t64;
        let t72 = 1.0 / t71;
        let t75 = 1.0 + 250.0 / 243.0 * t63 * t65 + 62500.0 / 59049.0 * param_b * t68 * t53 * t72;
        let t76 = 1.0 / t75;
        let t77 = t62 * t76;
        let t78 = param_kappa + t42 + param_c;
        let t83 = param_kappa * (1.0 - param_kappa / t78) - t47;
        let t85 = t77 * t83 + t47 + 1.0;
        let t89 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t85);
        let t90 = rho1 <= dens_threshold;
        let t91 = -t17;
        let t93 = piecewise5::<f64>(t15, t12, t11, t16, t91 * t8);
        let t94 = 1.0 + t93;
        let t95 = t94 <= zeta_threshold;
        let t96 = pow_1_3::<f64>(t94);
        let t98 = piecewise3::<f64>(t95, t23, t96 * t94);
        let t99 = t98 * t27;
        let t100 = rho1 * rho1;
        let t101 = pow_1_3::<f64>(rho1);
        let t102 = t101 * t101;
        let t104 = 1.0 / t102 / t100;
        let t105 = sigma2 * t104;
        let t107 = 5.0 / 972.0 * t34 * t105;
        let t108 = param_kappa + t107;
        let t112 = param_kappa * (1.0 - param_kappa / t108);
        let t114 = 1.0 / t102 / rho1;
        let t117 = tau1 * t114 - t105 / 8.0;
        let t118 = t117 * t117;
        let t122 = 1.0 - 25.0 / 81.0 * t118 * t54 * t57;
        let t123 = t122 * t122;
        let t124 = t123 * t122;
        let t125 = t118 * t117;
        let t128 = t118 * t118;
        let t133 = 1.0 + 250.0 / 243.0 * t125 * t65 + 62500.0 / 59049.0 * param_b * t128 * t118 * t72;
        let t134 = 1.0 / t133;
        let t135 = t124 * t134;
        let t136 = param_kappa + t107 + param_c;
        let t141 = param_kappa * (1.0 - param_kappa / t136) - t112;
        let t143 = t135 * t141 + t112 + 1.0;
        let t147 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t99 * t143);
        let tzk0 = t89 + t147;
        zk[ip] += tzk0;
        let t148 = t7 * t7;
        let t149 = 1.0 / t148;
        let t150 = t17 * t149;
        let t152 = piecewise5::<f64>(t11, 0.0, t15, 0.0, t8 - t150);
        let t155 = piecewise3::<f64>(t21, 0.0, 4.0 / 3.0 * t24 * t152);
        let t156 = t155 * t27;
        let t160 = t27 * t27;
        let t161 = 1.0 / t160;
        let t162 = t26 * t161;
        let t165 = t6 * t162 * t85 / 8.0;
        let t166 = param_kappa * param_kappa;
        let t167 = t43 * t43;
        let t169 = t166 / t167;
        let t170 = t169 * t29;
        let t171 = t33 * sigma0;
        let t172 = t35 * rho0;
        let t174 = 1.0 / t37 / t172;
        let t175 = t171 * t174;
        let t176 = t170 * t175;
        let t178 = t61 * t76;
        let t179 = t178 * t83;
        let t180 = t52 * t54;
        let t185 = -5.0 / 3.0 * tau0 * t39 + sigma0 * t174 / 3.0;
        let t186 = t57 * t185;
        let t187 = t180 * t186;
        let t190 = t75 * t75;
        let t191 = 1.0 / t190;
        let t192 = t62 * t191;
        let t193 = t53 * t65;
        let t197 = param_b * t68 * t52;
        let t198 = t72 * t185;
        let t201 = 250.0 / 81.0 * t193 * t185 + 125000.0 / 19683.0 * t197 * t198;
        let t202 = t83 * t201;
        let t204 = t78 * t78;
        let t206 = t166 / t204;
        let t207 = t206 * t29;
        let t210 = -10.0 / 729.0 * t207 * t175 + 10.0 / 729.0 * t176;
        let t212 = -10.0 / 729.0 * t176 - 50.0 / 27.0 * t179 * t187 - t192 * t202 + t77 * t210;
        let t217 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t156 * t85 - t165 - 3.0 / 8.0 * t6 * t28 * t212);
        let t218 = t91 * t149;
        let t220 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -t8 - t218);
        let t223 = piecewise3::<f64>(t95, 0.0, 4.0 / 3.0 * t96 * t220);
        let t224 = t223 * t27;
        let t228 = t98 * t161;
        let t231 = t6 * t228 * t143 / 8.0;
        let t233 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t224 * t143 - t231);
        let tvrho0 = t89 + t147 + t7 * (t217 + t233);
        vrho[ip * 2] += tvrho0;
        let t237 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -t8 - t150);
        let t240 = piecewise3::<f64>(t21, 0.0, 4.0 / 3.0 * t24 * t237);
        let t241 = t240 * t27;
        let t246 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t241 * t85 - t165);
        let t248 = piecewise5::<f64>(t15, 0.0, t11, 0.0, t8 - t218);
        let t251 = piecewise3::<f64>(t95, 0.0, 4.0 / 3.0 * t96 * t248);
        let t252 = t251 * t27;
        let t256 = t108 * t108;
        let t258 = t166 / t256;
        let t259 = t258 * t29;
        let t260 = t33 * sigma2;
        let t261 = t100 * rho1;
        let t263 = 1.0 / t102 / t261;
        let t264 = t260 * t263;
        let t265 = t259 * t264;
        let t267 = t123 * t134;
        let t268 = t267 * t141;
        let t269 = t117 * t54;
        let t274 = -5.0 / 3.0 * tau1 * t104 + sigma2 * t263 / 3.0;
        let t275 = t57 * t274;
        let t276 = t269 * t275;
        let t279 = t133 * t133;
        let t280 = 1.0 / t279;
        let t281 = t124 * t280;
        let t282 = t118 * t65;
        let t286 = param_b * t128 * t117;
        let t287 = t72 * t274;
        let t290 = 250.0 / 81.0 * t282 * t274 + 125000.0 / 19683.0 * t286 * t287;
        let t291 = t141 * t290;
        let t293 = t136 * t136;
        let t295 = t166 / t293;
        let t296 = t295 * t29;
        let t299 = -10.0 / 729.0 * t296 * t264 + 10.0 / 729.0 * t265;
        let t301 = -10.0 / 729.0 * t265 - 50.0 / 27.0 * t268 * t276 - t281 * t291 + t135 * t299;
        let t306 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t252 * t143 - t231 - 3.0 / 8.0 * t6 * t99 * t301);
        let tvrho1 = t89 + t147 + t7 * (t246 + t306);
        vrho[ip * 2 + 1] += tvrho1;
        let t309 = t34 * t39;
        let t310 = t169 * t309;
        let t312 = t57 * t39;
        let t313 = t180 * t312;
        let t314 = t179 * t313;
        let t316 = t193 * t39;
        let t318 = t72 * t39;
        let t319 = t197 * t318;
        let t321 = -125.0 / 324.0 * t316 - 15625.0 / 19683.0 * t319;
        let t322 = t83 * t321;
        let t326 = 5.0 / 972.0 * t206 * t309 - 5.0 / 972.0 * t310;
        let t328 = 5.0 / 972.0 * t310 + 25.0 / 108.0 * t314 - t192 * t322 + t77 * t326;
        let t332 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t328);
        let tvsigma0 = t7 * t332;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t333 = t34 * t104;
        let t334 = t258 * t333;
        let t336 = t57 * t104;
        let t337 = t269 * t336;
        let t338 = t268 * t337;
        let t340 = t282 * t104;
        let t342 = t72 * t104;
        let t343 = t286 * t342;
        let t345 = -125.0 / 324.0 * t340 - 15625.0 / 19683.0 * t343;
        let t346 = t141 * t345;
        let t350 = 5.0 / 972.0 * t295 * t333 - 5.0 / 972.0 * t334;
        let t352 = 5.0 / 972.0 * t334 + 25.0 / 108.0 * t338 - t281 * t346 + t135 * t350;
        let t356 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t99 * t352);
        let tvsigma2 = t7 * t356;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t357 = t57 * t49;
        let t358 = t180 * t357;
        let t363 = t72 * t49;
        let t366 = 250.0 / 81.0 * t193 * t49 + 125000.0 / 19683.0 * t197 * t363;
        let t367 = t83 * t366;
        let t369 = -50.0 / 27.0 * t179 * t358 - t192 * t367;
        let t373 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t369);
        let tvtau0 = t7 * t373;
        vtau[ip * 2] += tvtau0;
        let t374 = t57 * t114;
        let t375 = t269 * t374;
        let t380 = t72 * t114;
        let t383 = 250.0 / 81.0 * t282 * t114 + 125000.0 / 19683.0 * t286 * t380;
        let t384 = t141 * t383;
        let t386 = -50.0 / 27.0 * t268 * t375 - t281 * t384;
        let t390 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t99 * t386);
        let tvtau1 = t7 * t390;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
