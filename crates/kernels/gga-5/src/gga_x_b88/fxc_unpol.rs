//! GGA_X_B88 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 38 shared lines across all orders.
//! Delta: 44 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_b88_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
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
        let t20 = t3 * t3;
        let t21 = param_beta * t20;
        let t23 = pow_1_3(1.0 / M_PI);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t18 * t18;
        let t34 = 1.0 / t32 / t31;
        let t35 = param_gamma * param_beta;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t35 * t36;
        let t39 = 1.0 / t18 / rho[ip];
        let t43 = f64::ln(t36 * t28 * t39 + f64::sqrt(pow_2(t36 * t28 * t39) + 1.0));
        let t44 = t28 * t39 * t43;
        let t46 = t37 * t44 + 1.0;
        let t47 = 1.0 / t46;
        let t48 = t34 * t47;
        let t52 = 1.0 + 2.0 / 9.0 * t27 * t30 * t48;
        let t56 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        // --- vxc delta (27 lines) ---
        let t58 = t17 / t32;
        let t62 = t31 * rho[ip];
        let t64 = 1.0 / t32 / t62;
        let t65 = t64 * t47;
        let t69 = t46 * t46;
        let t70 = 1.0 / t69;
        let t71 = t34 * t70;
        let t75 = t28 / t18 / t31 * t43;
        let t77 = t35 * sigma[ip];
        let t78 = t29 * t64;
        let t80 = t30 * t34 + 1.0;
        let t81 = f64::sqrt(t80);
        let t82 = 1.0 / t81;
        let t83 = t78 * t82;
        let t86 = -4.0 / 3.0 * t37 * t75 - 4.0 / 3.0 * t77 * t83;
        let t91 = -16.0 / 27.0 * t27 * t30 * t65 - 2.0 / 9.0 * t27 * t30 * t71 * t86;
        let t96 = piecewise3(t2, 0.0, -t6 * t58 * t52 / 8.0 - 3.0 / 8.0 * t6 * t19 * t91);
        let tvrho0 = 2.0 * rho[ip] * t96 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t99 = t21 * t24;
        let t100 = t25 * t29;
        let t104 = t35 / t36;
        let t106 = t29 * t34;
        let t107 = t106 * t82;
        let t110 = t104 * t44 / 2.0 + t35 * t107 / 2.0;
        let t115 = -2.0 / 9.0 * t27 * t30 * t71 * t110 + 2.0 / 9.0 * t99 * t100 * t48;
        let t119 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t115);
        let tvsigma0 = 2.0 * rho[ip] * t119;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (44 lines) ---
        let t124 = t17 / t32 / rho[ip];
        let t131 = t31 * t31;
        let t133 = 1.0 / t32 / t131;
        let t134 = t133 * t47;
        let t138 = t64 * t70;
        let t144 = 1.0 / t69 / t46;
        let t145 = t34 * t144;
        let t146 = t86 * t86;
        let t154 = t28 / t18 / t62 * t43;
        let t157 = t29 * t133;
        let t158 = t157 * t82;
        let t161 = sigma[ip] * sigma[ip];
        let t162 = t35 * t161;
        let t165 = 1.0 / t18 / t131 / t62;
        let t168 = 1.0 / t81 / t80;
        let t169 = t28 * t165 * t168;
        let t172 = 28.0 / 9.0 * t37 * t154 + 20.0 / 3.0 * t77 * t158 - 32.0 / 9.0 * t162 * t169;
        let t177 = 176.0 / 81.0 * t27 * t30 * t134 + 32.0 / 27.0 * t27 * t30 * t138 * t86 + 4.0 / 9.0 * t27 * t30 * t145 * t146 - 2.0 / 9.0 * t27 * t30 * t71 * t172;
        let t182 = piecewise3(t2, 0.0, t6 * t124 * t52 / 12.0 - t6 * t58 * t91 / 4.0 - 3.0 / 8.0 * t6 * t19 * t177);
        let tv2rho20 = 2.0 * rho[ip] * t182 + 4.0 * t96;
        v2rho2[ip] += tv2rho20;
        let t191 = t70 * t86;
        let t200 = t21 * t26 * sigma[ip];
        let t201 = t144 * t110;
        let t202 = t201 * t86;
        let t203 = t106 * t202;
        let t210 = t35 * t28;
        let t211 = t131 * t31;
        let t213 = 1.0 / t18 / t211;
        let t218 = -2.0 / 3.0 * t104 * t75 - 2.0 * t35 * t83 + 4.0 / 3.0 * t210 * t213 * t168 * sigma[ip];
        let t223 = -16.0 / 27.0 * t99 * t100 * t65 - 2.0 / 9.0 * t27 * t106 * t191 + 16.0 / 27.0 * t27 * t30 * t138 * t110 + 4.0 / 9.0 * t200 * t203 - 2.0 / 9.0 * t27 * t30 * t71 * t218;
        let t228 = piecewise3(t2, 0.0, -t6 * t58 * t115 / 8.0 - 3.0 / 8.0 * t6 * t19 * t223);
        let tv2rhosigma0 = 2.0 * rho[ip] * t228 + 2.0 * t119;
        v2rhosigma[ip] += tv2rhosigma0;
        let t231 = t70 * t110;
        let t235 = t110 * t110;
        let t242 = t35 / t36 / sigma[ip];
        let t245 = 1.0 / sigma[ip];
        let t246 = t35 * t245;
        let t249 = t131 * rho[ip];
        let t252 = t28 / t18 / t249;
        let t253 = t252 * t168;
        let t256 = -t242 * t44 / 4.0 + t246 * t107 / 4.0 - t35 * t253 / 2.0;
        let t261 = -4.0 / 9.0 * t27 * t106 * t231 + 4.0 / 9.0 * t27 * t30 * t145 * t235 - 2.0 / 9.0 * t27 * t30 * t71 * t256;
        let t265 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t261);
        let tv2sigma20 = 2.0 * rho[ip] * t265;
        v2sigma2[ip] += tv2sigma20;
    }
}
