//! MGGA_X_GX vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 88 shared lines across all orders.
//! Delta: 128 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gx_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_alphainf: f64,
    param_c0: f64,
    param_c1: f64,
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
        // --- shared preamble (88 lines) ---
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
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT2;
        let t30 = t3 * t3;
        let t32 = M_CBRT4;
        let t34 = 8.0 / 27.0 * t29 * t30 * t32;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / rho0;
        let t40 = rho0 * rho0;
        let t42 = 1.0 / t36 / t40;
        let t45 = tau0 * t38 - sigma0 * t42 / 8.0;
        let t46 = M_CBRT6;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t45 * t46 * t51;
        let t54 = t46 * t51;
        let t57 = param_c0 + 5.0 / 9.0 * param_c1 * t45 * t54;
        let t58 = param_c0 + param_c1 - 1.0;
        let t62 = 1.0 + 5.0 / 9.0 * t58 * t45 * t54;
        let t63 = 1.0 / t62;
        let t65 = 1.0 - t34;
        let t66 = t57 * t63 * t65;
        let t69 = t34 + 5.0 / 9.0 * t52 * t66;
        let t70 = 5.0 / 9.0 * t52;
        let t71 = 1.0 - t70;
        let t72 = Heaviside(t71);
        let t74 = 1.0 - param_alphainf;
        let t75 = t74 * t71;
        let t76 = 1.0 + t70;
        let t77 = 1.0 / t76;
        let t79 = t75 * t77 + 1.0;
        let t80 = -t71;
        let t81 = Heaviside(t80);
        let t83 = t69 * t72 + t79 * t81;
        let t87 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t83);
        let t88 = rho1 <= dens_threshold;
        let t89 = -t17;
        let t91 = piecewise5(t15, t12, t11, t16, t89 * t8);
        let t92 = 1.0 + t91;
        let t93 = t92 <= zeta_threshold;
        let t94 = pow_1_3(t92);
        let t96 = piecewise3(t93, t23, t94 * t92);
        let t97 = t96 * t27;
        let t98 = pow_1_3(rho1);
        let t99 = t98 * t98;
        let t101 = 1.0 / t99 / rho1;
        let t103 = rho1 * rho1;
        let t105 = 1.0 / t99 / t103;
        let t108 = tau1 * t101 - sigma2 * t105 / 8.0;
        let t110 = t108 * t46 * t51;
        let t114 = param_c0 + 5.0 / 9.0 * param_c1 * t108 * t54;
        let t118 = 1.0 + 5.0 / 9.0 * t58 * t108 * t54;
        let t119 = 1.0 / t118;
        let t121 = t114 * t119 * t65;
        let t124 = t34 + 5.0 / 9.0 * t110 * t121;
        let t125 = 5.0 / 9.0 * t110;
        let t126 = 1.0 - t125;
        let t127 = Heaviside(t126);
        let t129 = t74 * t126;
        let t130 = 1.0 + t125;
        let t131 = 1.0 / t130;
        let t133 = t129 * t131 + 1.0;
        let t134 = -t126;
        let t135 = Heaviside(t134);
        let t137 = t124 * t127 + t133 * t135;
        let t141 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t97 * t137);
        let tzk0 = t87 + t141;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (128 lines) ---
        let t142 = t7 * t7;
        let t143 = 1.0 / t142;
        let t144 = t17 * t143;
        let t146 = piecewise5(t11, 0.0, t15, 0.0, t8 - t144);
        let t149 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t146);
        let t150 = t149 * t27;
        let t154 = t27 * t27;
        let t155 = 1.0 / t154;
        let t156 = t26 * t155;
        let t159 = t6 * t156 * t83 / 8.0;
        let t162 = t40 * rho0;
        let t164 = 1.0 / t36 / t162;
        let t167 = -5.0 / 3.0 * tau0 * t42 + sigma0 * t164 / 3.0;
        let t169 = t167 * t46 * t51;
        let t172 = t46 * t46;
        let t173 = t45 * t172;
        let t175 = 1.0 / t49 / t48;
        let t176 = t173 * t175;
        let t177 = param_c1 * t167;
        let t178 = t63 * t65;
        let t179 = t177 * t178;
        let t182 = t175 * t57;
        let t183 = t173 * t182;
        let t184 = t62 * t62;
        let t185 = 1.0 / t184;
        let t186 = t185 * t65;
        let t188 = t186 * t58 * t167;
        let t191 = 5.0 / 9.0 * t169 * t66 + 25.0 / 81.0 * t176 * t179 - 25.0 / 81.0 * t183 * t188;
        let t193 = 0.0;
        let t194 = t69 * t193;
        let t198 = t54 * t77;
        let t200 = t76 * t76;
        let t201 = 1.0 / t200;
        let t202 = t75 * t201;
        let t205 = -5.0 / 9.0 * t74 * t167 * t198 - 5.0 / 9.0 * t202 * t169;
        let t207 = t79 * t193;
        let t210 = t191 * t72 - 5.0 / 9.0 * t194 * t169 + t205 * t81 + 5.0 / 9.0 * t207 * t169;
        let t215 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t150 * t83 - t159 - 3.0 / 8.0 * t6 * t28 * t210);
        let t216 = t89 * t143;
        let t218 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t216);
        let t221 = piecewise3(t93, 0.0, 4.0 / 3.0 * t94 * t218);
        let t222 = t221 * t27;
        let t226 = t96 * t155;
        let t229 = t6 * t226 * t137 / 8.0;
        let t231 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t222 * t137 - t229);
        let tvrho0 = t87 + t141 + t7 * (t215 + t231);
        vrho[ip * 2] += tvrho0;
        let t235 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t144);
        let t238 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t235);
        let t239 = t238 * t27;
        let t244 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t239 * t83 - t159);
        let t246 = piecewise5(t15, 0.0, t11, 0.0, t8 - t216);
        let t249 = piecewise3(t93, 0.0, 4.0 / 3.0 * t94 * t246);
        let t250 = t249 * t27;
        let t256 = t103 * rho1;
        let t258 = 1.0 / t99 / t256;
        let t261 = -5.0 / 3.0 * tau1 * t105 + sigma2 * t258 / 3.0;
        let t263 = t261 * t46 * t51;
        let t266 = t108 * t172;
        let t267 = t266 * t175;
        let t269 = t119 * t65;
        let t270 = param_c1 * t261 * t269;
        let t273 = t175 * t114;
        let t274 = t266 * t273;
        let t275 = t118 * t118;
        let t276 = 1.0 / t275;
        let t277 = t276 * t65;
        let t279 = t277 * t58 * t261;
        let t282 = 5.0 / 9.0 * t263 * t121 + 25.0 / 81.0 * t267 * t270 - 25.0 / 81.0 * t274 * t279;
        let t284 = 0.0;
        let t285 = t124 * t284;
        let t289 = t54 * t131;
        let t291 = t130 * t130;
        let t292 = 1.0 / t291;
        let t293 = t129 * t292;
        let t296 = -5.0 / 9.0 * t74 * t261 * t289 - 5.0 / 9.0 * t293 * t263;
        let t298 = t133 * t284;
        let t301 = t282 * t127 - 5.0 / 9.0 * t285 * t263 + t296 * t135 + 5.0 / 9.0 * t298 * t263;
        let t306 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t250 * t137 - t229 - 3.0 / 8.0 * t6 * t97 * t301);
        let tvrho1 = t87 + t141 + t7 * (t244 + t306);
        vrho[ip * 2 + 1] += tvrho1;
        let t310 = t42 * t46 * t51;
        let t311 = t310 * t66;
        let t313 = param_c1 * t42;
        let t315 = t176 * t313 * t178;
        let t319 = t183 * t186 * t58 * t42;
        let t321 = -5.0 / 72.0 * t311 - 25.0 / 648.0 * t315 + 25.0 / 648.0 * t319;
        let t323 = t194 * t310;
        let t325 = t74 * t42;
        let t326 = t325 * t198;
        let t327 = t202 * t310;
        let t329 = 5.0 / 72.0 * t326 + 5.0 / 72.0 * t327;
        let t331 = t207 * t310;
        let t333 = t321 * t72 + 5.0 / 72.0 * t323 + t329 * t81 - 5.0 / 72.0 * t331;
        let t337 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t333);
        let tvsigma0 = t7 * t337;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t339 = t105 * t46 * t51;
        let t340 = t339 * t121;
        let t342 = param_c1 * t105;
        let t344 = t267 * t342 * t269;
        let t348 = t274 * t277 * t58 * t105;
        let t350 = -5.0 / 72.0 * t340 - 25.0 / 648.0 * t344 + 25.0 / 648.0 * t348;
        let t352 = t285 * t339;
        let t354 = t74 * t105;
        let t355 = t354 * t289;
        let t356 = t293 * t339;
        let t358 = 5.0 / 72.0 * t355 + 5.0 / 72.0 * t356;
        let t360 = t298 * t339;
        let t362 = t350 * t127 + 5.0 / 72.0 * t352 + t358 * t135 - 5.0 / 72.0 * t360;
        let t366 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t97 * t362);
        let tvsigma2 = t7 * t366;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t368 = t38 * t46 * t51;
        let t371 = param_c1 * t38;
        let t379 = 5.0 / 9.0 * t368 * t66 + 25.0 / 81.0 * t176 * t371 * t178 - 25.0 / 81.0 * t183 * t186 * t58 * t38;
        let t383 = t74 * t38;
        let t387 = -5.0 / 9.0 * t383 * t198 - 5.0 / 9.0 * t202 * t368;
        let t391 = t379 * t72 - 5.0 / 9.0 * t194 * t368 + t387 * t81 + 5.0 / 9.0 * t207 * t368;
        let t395 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t391);
        let tvtau0 = t7 * t395;
        vtau[ip * 2] += tvtau0;
        let t397 = t101 * t46 * t51;
        let t400 = param_c1 * t101;
        let t408 = 5.0 / 9.0 * t397 * t121 + 25.0 / 81.0 * t267 * t400 * t269 - 25.0 / 81.0 * t274 * t277 * t58 * t101;
        let t412 = t74 * t101;
        let t416 = -5.0 / 9.0 * t412 * t289 - 5.0 / 9.0 * t293 * t397;
        let t420 = t408 * t127 - 5.0 / 9.0 * t285 * t397 + t416 * t135 + 5.0 / 9.0 * t298 * t397;
        let t424 = piecewise3(t88, 0.0, -3.0 / 8.0 * t6 * t97 * t420);
        let tvtau1 = t7 * t424;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
