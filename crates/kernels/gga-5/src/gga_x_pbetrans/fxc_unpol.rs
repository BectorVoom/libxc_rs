//! GGA_X_PBETRANS fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 38 shared lines across all orders.
//! Delta: 81 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbetrans_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (38 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_PI * M_PI;
        let t21 = pow_1_3(t20);
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t27 = f64::sqrt(sigma[ip]);
        let t28 = M_CBRT2;
        let t29 = t27 * t28;
        let t31 = 1.0 / t18 / rho[ip];
        let t38 = f64::exp(-2.0 * t3 * t21 * (t24 / t21 * t29 * t31 / 12.0 - 3.0));
        let t39 = 1.0 + t38;
        let t41 = 0.413e0 / t39;
        let t42 = 0.1227e1 - t41;
        let t43 = t21 * t21;
        let t45 = t23 / t43;
        let t46 = t28 * t28;
        let t47 = sigma[ip] * t46;
        let t48 = rho[ip] * rho[ip];
        let t49 = t18 * t18;
        let t51 = 1.0 / t49 / t48;
        let t55 = 0.1227e1 - t41 + 0.91249999999999999998e-2 * t45 * t47 * t51;
        let t56 = 1.0 / t55;
        let t58 = -t42 * t56 + 1.0;
        let t60 = t42 * t58 + 1.0;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        // --- vxc delta (38 lines) ---
        let t66 = t17 / t49;
        let t70 = t39 * t39;
        let t71 = 1.0 / t70;
        let t72 = t71 * t3;
        let t73 = t24 * t27;
        let t74 = t72 * t73;
        let t76 = 1.0 / t18 / t48;
        let t77 = t28 * t76;
        let t78 = t38 * t58;
        let t79 = t77 * t78;
        let t82 = t38 * t56;
        let t83 = t77 * t82;
        let t86 = t55 * t55;
        let t87 = 1.0 / t86;
        let t88 = t42 * t87;
        let t89 = t72 * t24;
        let t90 = t76 * t38;
        let t94 = t48 * rho[ip];
        let t96 = 1.0 / t49 / t94;
        let t100 = 0.91777777777777777778e-1 * t89 * t29 * t90 - 0.24333333333333333333e-1 * t45 * t47 * t96;
        let t102 = -0.91777777777777777778e-1 * t74 * t83 + t88 * t100;
        let t104 = 0.91777777777777777778e-1 * t74 * t79 + t42 * t102;
        let t109 = piecewise3(t2, 0.0, -t6 * t66 * t60 / 8.0 - 3.0 / 8.0 * t6 * t19 * t104);
        let tvrho0 = 2.0 * rho[ip] * t109 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t112 = 1.0 / t27;
        let t113 = t24 * t112;
        let t114 = t72 * t113;
        let t115 = t28 * t31;
        let t116 = t115 * t78;
        let t119 = t115 * t82;
        let t122 = t112 * t28;
        let t123 = t31 * t38;
        let t127 = t46 * t51;
        let t130 = -0.34416666666666666667e-1 * t89 * t122 * t123 + 0.91249999999999999998e-2 * t45 * t127;
        let t132 = 0.34416666666666666667e-1 * t114 * t119 + t88 * t130;
        let t134 = -0.34416666666666666667e-1 * t114 * t116 + t42 * t132;
        let t138 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t134);
        let tvsigma0 = 2.0 * rho[ip] * t138;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (81 lines) ---
        let t143 = t17 / t49 / rho[ip];
        let t151 = 1.0 / t70 / t39;
        let t152 = t3 * t3;
        let t153 = t151 * t152;
        let t154 = t23 * sigma[ip];
        let t155 = t153 * t154;
        let t156 = t48 * t48;
        let t158 = 1.0 / t49 / t156;
        let t159 = t46 * t158;
        let t160 = t38 * t38;
        let t161 = t160 * t58;
        let t162 = t159 * t161;
        let t166 = 1.0 / t18 / t94;
        let t167 = t28 * t166;
        let t168 = t167 * t78;
        let t171 = t71 * t152;
        let t172 = t171 * t154;
        let t173 = t159 * t78;
        let t176 = t38 * t102;
        let t177 = t77 * t176;
        let t180 = t160 * t56;
        let t181 = t159 * t180;
        let t184 = t167 * t82;
        let t187 = t159 * t82;
        let t190 = t38 * t87;
        let t191 = t190 * t100;
        let t192 = t77 * t191;
        let t196 = 1.0 / t86 / t55;
        let t197 = t42 * t196;
        let t198 = t100 * t100;
        let t201 = t153 * t23;
        let t202 = t158 * t160;
        let t206 = t166 * t38;
        let t210 = t171 * t23;
        let t211 = t158 * t38;
        let t218 = -0.24474074074074074074e0 * t201 * t47 * t202 - 0.21414814814814814815e0 * t89 * t29 * t206 + 0.12237037037037037037e0 * t210 * t47 * t211 + 0.89222222222222222221e-1 * t45 * t47 * t158;
        let t220 = 0.24474074074074074074e0 * t155 * t181 + 0.21414814814814814815e0 * t74 * t184 - 0.12237037037037037037e0 * t172 * t187 + 0.18355555555555555556e0 * t74 * t192 - 2.0 * t197 * t198 + t88 * t218;
        let t222 = -0.24474074074074074074e0 * t155 * t162 - 0.21414814814814814815e0 * t74 * t168 + 0.12237037037037037037e0 * t172 * t173 + 0.18355555555555555556e0 * t74 * t177 + t42 * t220;
        let t227 = piecewise3(t2, 0.0, t6 * t143 * t60 / 12.0 - t6 * t66 * t104 / 4.0 - 3.0 / 8.0 * t6 * t19 * t222);
        let tv2rho20 = 2.0 * rho[ip] * t227 + 4.0 * t109;
        v2rho2[ip] += tv2rho20;
        let t233 = t46 * t96;
        let t234 = t233 * t161;
        let t239 = t233 * t78;
        let t242 = t115 * t176;
        let t245 = t38 * t132;
        let t246 = t77 * t245;
        let t249 = t233 * t180;
        let t254 = t233 * t82;
        let t257 = t115 * t191;
        let t260 = t190 * t130;
        let t261 = t77 * t260;
        let t264 = t130 * t100;
        let t278 = 0.91777777777777777779e-1 * t201 * t233 * t160 + 0.45888888888888888889e-1 * t89 * t122 * t90 - 0.45888888888888888889e-1 * t210 * t233 * t38 - 0.24333333333333333333e-1 * t45 * t233;
        let t280 = -0.91777777777777777779e-1 * t201 * t249 - 0.45888888888888888889e-1 * t114 * t83 + 0.45888888888888888889e-1 * t210 * t254 - 0.34416666666666666667e-1 * t114 * t257 + 0.91777777777777777778e-1 * t74 * t261 - 2.0 * t197 * t264 + t88 * t278;
        let t282 = 0.91777777777777777779e-1 * t201 * t234 + 0.45888888888888888889e-1 * t114 * t79 - 0.45888888888888888889e-1 * t210 * t239 - 0.34416666666666666667e-1 * t114 * t242 + 0.91777777777777777778e-1 * t74 * t246 + t42 * t280;
        let t287 = piecewise3(t2, 0.0, -t6 * t66 * t134 / 8.0 - 3.0 / 8.0 * t6 * t19 * t282);
        let tv2rhosigma0 = 2.0 * rho[ip] * t287 + 2.0 * t138;
        v2rhosigma[ip] += tv2rhosigma0;
        let t290 = 1.0 / sigma[ip];
        let t291 = t23 * t290;
        let t292 = t153 * t291;
        let t293 = t127 * t161;
        let t296 = t27 * sigma[ip];
        let t297 = 1.0 / t296;
        let t298 = t24 * t297;
        let t299 = t72 * t298;
        let t302 = t171 * t291;
        let t303 = t127 * t78;
        let t306 = t115 * t245;
        let t309 = t127 * t180;
        let t314 = t127 * t82;
        let t317 = t115 * t260;
        let t320 = t130 * t130;
        let t323 = t290 * t46;
        let t324 = t51 * t160;
        let t328 = t297 * t28;
        let t332 = t51 * t38;
        let t336 = -0.34416666666666666667e-1 * t201 * t323 * t324 + 0.17208333333333333334e-1 * t89 * t328 * t123 + 0.17208333333333333334e-1 * t210 * t323 * t332;
        let t338 = 0.34416666666666666667e-1 * t292 * t309 - 0.17208333333333333334e-1 * t299 * t119 - 0.17208333333333333334e-1 * t302 * t314 - 0.68833333333333333334e-1 * t114 * t317 - 2.0 * t197 * t320 + t88 * t336;
        let t340 = -0.34416666666666666667e-1 * t292 * t293 + 0.17208333333333333334e-1 * t299 * t116 + 0.17208333333333333334e-1 * t302 * t303 - 0.68833333333333333334e-1 * t114 * t306 + t42 * t338;
        let t344 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t340);
        let tv2sigma20 = 2.0 * rho[ip] * t344;
        v2sigma2[ip] += tv2sigma20;
    }
}
