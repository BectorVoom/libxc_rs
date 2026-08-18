//! MGGA_X_TM vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tm_vxc_unpol(
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
        let t66 = f64::powf(t65, 1.0 / 5.0);
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
        let t101 = (t72 - t49 / 8.0) * t36 * t40 / 4.0 - 9.0 / 20.0 + t50 / 36.0;
        let t102 = t101 * t101;
        let t104 = t101 * t27;
        let t105 = 1.0 - t27;
        let t108 = 1.0 + 5.0 / 12.0 * t92 * t49 + 292.0 / 405.0 * t102 - 146.0 / 135.0 * t104 * t105;
        let t109 = f64::powf(t108, 1.0 / 10.0);
        let t111 = t88 * t109 + t35 * t86;
        let t115 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t111);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
        let t117 = t18 / t46;
        let t121 = 1.0 / t45;
        let t122 = sigma[ip] * t121;
        let t125 = piecewise3(t26, -t122 * t23 / 8.0, 0.0);
        let t126 = t27 * t125;
        let t128 = t28 * t125;
        let t130 = 2.0 * t126 + 9.0 * t128;
        let t131 = t130 * t34;
        let t134 = 1.0 / t33 / t32;
        let t135 = t31 * t134;
        let t136 = t86 * t28;
        let t137 = t136 * t125;
        let t141 = 1.0 / t66 / t65;
        let t142 = t45 * rho[ip];
        let t144 = 1.0 / t46 / t142;
        let t145 = t44 * t144;
        let t146 = t41 * t145;
        let t148 = t58 * t45;
        let t150 = 1.0 / t19 / t148;
        let t152 = t55 * t57 * t150;
        let t154 = -0.40121303703703703 * t146 - 0.028692789826413812 * t152;
        let t158 = t69 * t48;
        let t165 = -0.17051554074074074 * t146 - 5.0 / 9.0 * (-0.24256886666666666 * t158 - 0.031646617777777775 * t145) * t36 * t40;
        let t169 = 1.0 / t82 / t65;
        let t170 = t81 * t169;
        let t173 = -t141 * t154 / 5.0 + 7.0 / 9.0 * t165 * t83 - 14.0 / 45.0 * t170 * t154;
        let t177 = 6.0 * t135 * t128 - t131;
        let t179 = t109 * t109;
        let t180 = t179 * t179;
        let t181 = t180 * t180;
        let t182 = t181 * t109;
        let t183 = 1.0 / t182;
        let t184 = t88 * t183;
        let t195 = (-5.0 / 3.0 * t158 + t145 / 3.0) * t36 * t40 / 4.0 - 2.0 / 27.0 * t146;
        let t198 = t195 * t27;
        let t201 = t101 * t125;
        let t206 = -125.0 / 19683.0 * t152 - 10.0 / 9.0 * t92 * t145 + 584.0 / 405.0 * t101 * t195 - 146.0 / 135.0 * t198 * t105 - 146.0 / 135.0 * t201 * t105 + 146.0 / 135.0 * t104 * t125;
        let t209 = t131 * t86 - 6.0 * t135 * t137 + t35 * t173 + t177 * t109 + t184 * t206 / 10.0;
        let t214 = piecewise3(t3, 0.0, -t7 * t117 * t111 / 8.0 - 3.0 / 8.0 * t7 * t20 * t209);
        let tvrho0 = 2.0 * rho[ip] * t214 + 2.0 * t115;
        vrho[ip] += tvrho0;
        let t219 = piecewise3(t26, t21 * t23 / 8.0, 0.0);
        let t220 = t27 * t219;
        let t222 = t28 * t219;
        let t224 = 2.0 * t220 + 9.0 * t222;
        let t225 = t224 * t34;
        let t227 = t136 * t219;
        let t230 = t43 * t48;
        let t233 = sigma[ip] * t42;
        let t235 = t55 * t233 * t61;
        let t237 = 0.1504548888888889 * t41 * t230 + 0.01075979618490518 * t235;
        let t245 = -t141 * t237 / 5.0 + 0.04460577520576132 * t41 * t230 * t83 - 14.0 / 45.0 * t170 * t237;
        let t249 = 6.0 * t135 * t222 - t225;
        let t252 = t40 * t43;
        let t253 = t252 * t48;
        let t256 = t101 * t36;
        let t257 = t256 * t253;
        let t259 = t41 * t43;
        let t260 = t48 * t27;
        let t262 = t259 * t260 * t105;
        let t264 = t101 * t219;
        let t269 = 125.0 / 52488.0 * t235 + 5.0 / 12.0 * t91 * t253 - 73.0 / 14580.0 * t257 + 73.0 / 19440.0 * t262 - 146.0 / 135.0 * t264 * t105 + 146.0 / 135.0 * t104 * t219;
        let t272 = t225 * t86 - 6.0 * t135 * t227 + t35 * t245 + t249 * t109 + t184 * t269 / 10.0;
        let t276 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t272);
        let tvsigma0 = 2.0 * rho[ip] * t276;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t278 = tau[ip] * tau[ip];
        let t279 = 1.0 / t278;
        let t282 = piecewise3(t26, -t22 * t279 / 8.0, 0.0);
        let t283 = t27 * t282;
        let t285 = t28 * t282;
        let t287 = 2.0 * t283 + 9.0 * t285;
        let t288 = t287 * t34;
        let t290 = t136 * t282;
        let t293 = t35 * t43;
        let t294 = t71 * t36;
        let t295 = t40 * t83;
        let t296 = t294 * t295;
        let t301 = 6.0 * t135 * t285 - t288;
        let t304 = t294 * t40;
        let t307 = t43 * t71;
        let t308 = t307 * t36;
        let t309 = t40 * t27;
        let t313 = t101 * t282;
        let t318 = 146.0 / 405.0 * t101 * t43 * t304 - 73.0 / 270.0 * t308 * t309 * t105 - 146.0 / 135.0 * t313 * t105 + 146.0 / 135.0 * t104 * t282;
        let t321 = t288 * t86 - 6.0 * t135 * t290 - 0.06288822469135802 * t293 * t296 + t301 * t109 + t184 * t318 / 10.0;
        let t325 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t321);
        let tvtau0 = 2.0 * rho[ip] * t325;
        vtau[ip] += tvtau0;
    }
}
