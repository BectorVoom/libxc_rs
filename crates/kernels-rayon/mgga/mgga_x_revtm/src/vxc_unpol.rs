//! MGGA_X_REVTM vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_revtm_vxc_unpol(
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
    for ip in 0..zk.len() {
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
        let t21 = 1.0 / rho[ip];
        let t22 = sigma[ip] * t21;
        let t23 = 1.0 / tau[ip];
        let t25 = t22 * t23 / 8.0;
        let t26 = t25 < 1.0;
        let t27 = piecewise3(t26, t25, 1.0);
        let t28 = t27 * t27;
        let t29 = t28 * t27;
        let t31 = t28 + 3.0 * t29;
        let t32 = 1.0 + t29;
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t31 * t34;
        let t36 = M_CBRT6;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t36 * t40;
        let t42 = M_CBRT2;
        let t43 = t42 * t42;
        let t44 = sigma[ip] * t43;
        let t45 = rho[ip] * rho[ip];
        let t46 = t19 * t19;
        let t48 = 1.0 / t46 / t45;
        let t49 = t44 * t48;
        let t50 = t41 * t49;
        let t52 = t36 * t36;
        let t54 = 1.0 / t38 / t37;
        let t55 = t52 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t42;
        let t58 = t45 * t45;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t19 / t59;
        let t65 = 1.0 + 0.1504548888888889 * t50 + 0.00537989809245259 * t55 * t57 * t61;
        let t66 = rmath::pow(t65, 1.0 / 5.0);
        let t69 = tau[ip] * t43;
        let t71 = 1.0 / t46 / rho[ip];
        let t72 = t69 * t71;
        let t81 = 1.0 + 0.06394332777777778 * t50 - 5.0 / 9.0 * (0.14554132 * t72 + 0.256337604 * t52 * t39 + 0.011867481666666667 * t49) * t36 * t40;
        let t82 = t66 * t66;
        let t83 = 1.0 / t82;
        let t86 = 1.0 / t66 + 7.0 / 9.0 * t81 * t83;
        let t88 = 1.0 - t35;
        let t91 = (10.0 / 81.0 + 25.0 / 8748.0 * t50) * t36;
        let t92 = t91 * t40;
        let t96 = t72 - t49 / 8.0;
        let t97 = t96 * t36;
        let t100 = 5.0 / 9.0 * t97 * t40 - 1.0;
        let t101 = t40 * t100;
        let t104 = 1.0 + 0.2222222222222222 * t97 * t101;
        let t105 = rmath::sqrt(t104);
        let t106 = 1.0 / t105;
        let t110 = 9.0 / 20.0 * t100 * t106 + t50 / 36.0;
        let t111 = t110 * t110;
        let t113 = t110 * t27;
        let t114 = 1.0 - t27;
        let t117 = 1.0 + 5.0 / 12.0 * t92 * t49 + 292.0 / 405.0 * t111 - 146.0 / 135.0 * t113 * t114;
        let t118 = rmath::pow(t117, 1.0 / 10.0);
        let t120 = t88 * t118 + t35 * t86;
        let t124 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t120);
        let tzk0 = 2.0 * t124;
        zk[ip] += tzk0;
        let t126 = t18 / t46;
        let t130 = 1.0 / t45;
        let t131 = sigma[ip] * t130;
        let t134 = piecewise3(t26, -t131 * t23 / 8.0, 0.0);
        let t135 = t27 * t134;
        let t137 = t28 * t134;
        let t139 = 2.0 * t135 + 9.0 * t137;
        let t140 = t139 * t34;
        let t143 = 1.0 / t33 / t32;
        let t144 = t31 * t143;
        let t145 = t86 * t28;
        let t146 = t145 * t134;
        let t150 = 1.0 / t66 / t65;
        let t151 = t45 * rho[ip];
        let t153 = 1.0 / t46 / t151;
        let t154 = t44 * t153;
        let t155 = t41 * t154;
        let t157 = t58 * t45;
        let t159 = 1.0 / t19 / t157;
        let t161 = t55 * t57 * t159;
        let t163 = -0.40121303703703703 * t155 - 0.028692789826413812 * t161;
        let t167 = t69 * t48;
        let t174 = -0.17051554074074074 * t155 - 5.0 / 9.0 * (-0.24256886666666666 * t167 - 0.031646617777777775 * t154) * t36 * t40;
        let t178 = 1.0 / t82 / t65;
        let t179 = t81 * t178;
        let t182 = -t150 * t163 / 5.0 + 7.0 / 9.0 * t174 * t83 - 14.0 / 45.0 * t179 * t163;
        let t186 = 6.0 * t144 * t137 - t140;
        let t188 = t118 * t118;
        let t189 = t188 * t188;
        let t190 = t189 * t189;
        let t191 = t190 * t118;
        let t192 = 1.0 / t191;
        let t193 = t88 * t192;
        let t199 = -5.0 / 3.0 * t167 + t154 / 3.0;
        let t200 = t199 * t36;
        let t201 = t40 * t106;
        let t205 = 1.0 / t105 / t104;
        let t206 = t100 * t205;
        let t209 = t96 * t52;
        let t210 = t54 * t199;
        let t213 = 0.2222222222222222 * t200 * t101 + 0.12345679012345678 * t209 * t210;
        let t217 = t200 * t201 / 4.0 - 9.0 / 40.0 * t206 * t213 - 2.0 / 27.0 * t155;
        let t220 = t217 * t27;
        let t223 = t110 * t134;
        let t228 = -125.0 / 19683.0 * t161 - 10.0 / 9.0 * t92 * t154 + 584.0 / 405.0 * t110 * t217 - 146.0 / 135.0 * t220 * t114 - 146.0 / 135.0 * t223 * t114 + 146.0 / 135.0 * t113 * t134;
        let t231 = t140 * t86 - 6.0 * t144 * t146 + t35 * t182 + t186 * t118 + t193 * t228 / 10.0;
        let t236 = piecewise3(t3, 0.0, -t7 * t126 * t120 / 8.0 - 3.0 / 8.0 * t7 * t20 * t231);
        let tvrho0 = 2.0 * rho[ip] * t236 + 2.0 * t124;
        vrho[ip] += tvrho0;
        let t241 = piecewise3(t26, t21 * t23 / 8.0, 0.0);
        let t242 = t27 * t241;
        let t244 = t28 * t241;
        let t246 = 2.0 * t242 + 9.0 * t244;
        let t247 = t246 * t34;
        let t249 = t145 * t241;
        let t252 = t43 * t48;
        let t253 = t41 * t252;
        let t255 = sigma[ip] * t42;
        let t257 = t55 * t255 * t61;
        let t259 = 0.1504548888888889 * t253 + 0.01075979618490518 * t257;
        let t267 = -t150 * t259 / 5.0 + 0.04460577520576132 * t41 * t252 * t83 - 14.0 / 45.0 * t179 * t259;
        let t271 = 6.0 * t144 * t244 - t247;
        let t274 = t40 * t43;
        let t279 = t41 * t252 * t106;
        let t281 = t41 * t100;
        let t282 = t252 * t281;
        let t284 = t54 * t43;
        let t286 = t209 * t284 * t48;
        let t288 = -0.027777777777777776 * t282 - 0.015432098765432098 * t286;
        let t292 = -t279 / 32.0 - 9.0 / 40.0 * t206 * t288 + t253 / 36.0;
        let t295 = t292 * t27;
        let t298 = t110 * t241;
        let t303 = 125.0 / 52488.0 * t257 + 5.0 / 12.0 * t91 * t274 * t48 + 584.0 / 405.0 * t110 * t292 - 146.0 / 135.0 * t295 * t114 - 146.0 / 135.0 * t298 * t114 + 146.0 / 135.0 * t113 * t241;
        let t306 = t247 * t86 - 6.0 * t144 * t249 + t35 * t267 + t271 * t118 + t193 * t303 / 10.0;
        let t310 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t306);
        let tvsigma0 = 2.0 * rho[ip] * t310;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t312 = tau[ip] * tau[ip];
        let t313 = 1.0 / t312;
        let t316 = piecewise3(t26, -t22 * t313 / 8.0, 0.0);
        let t317 = t27 * t316;
        let t319 = t28 * t316;
        let t321 = 2.0 * t317 + 9.0 * t319;
        let t322 = t321 * t34;
        let t324 = t145 * t316;
        let t327 = t35 * t43;
        let t329 = t40 * t83;
        let t330 = t71 * t36 * t329;
        let t335 = 6.0 * t144 * t319 - t322;
        let t337 = t43 * t71;
        let t346 = 0.2222222222222222 * t337 * t281 + 0.12345679012345678 * t209 * t284 * t71;
        let t349 = t337 * t41 * t106 / 4.0 - 9.0 / 40.0 * t206 * t346;
        let t352 = t349 * t27;
        let t355 = t110 * t316;
        let t360 = 584.0 / 405.0 * t110 * t349 - 146.0 / 135.0 * t352 * t114 - 146.0 / 135.0 * t355 * t114 + 146.0 / 135.0 * t113 * t316;
        let t363 = t322 * t86 - 6.0 * t144 * t324 - 0.06288822469135802 * t327 * t330 + t335 * t118 + t193 * t360 / 10.0;
        let t367 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t363);
        let tvtau0 = 2.0 * rho[ip] * t367;
        vtau[ip] += tvtau0;
    }
}
