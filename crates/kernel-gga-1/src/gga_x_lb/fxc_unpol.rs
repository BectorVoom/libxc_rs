//! GGA_X_LB fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 29 shared lines across all orders.
//! Delta: 34 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lb_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        // --- shared preamble (29 lines) ---
        let t1 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t10 = f64::sqrt(sigma[ip]);
        let t11 = M_CBRT2;
        let t12 = t10 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t15 = 1.0 / t13 / rho[ip];
        let t17 = t12 * t15 < 300.0;
        let t18 = param_beta * sigma[ip];
        let t19 = t11 * t11;
        let t20 = rho[ip] * rho[ip];
        let t21 = t13 * t13;
        let t23 = 1.0 / t21 / t20;
        let t24 = t19 * t23;
        let t25 = param_beta * t10;
        let t26 = t11 * t15;
        let t28 = param_gamma * t10 * t26;
        let t29 = f64::ln(t28 + f64::sqrt(t28 * t28 + 1.0));
        let t30 = t26 * t29;
        let t33 = 3.0 * t25 * t30 + 1.0;
        let t34 = 1.0 / t33;
        let t38 = f64::ln(2.0 * t28);
        let t39 = 1.0 / t38;
        let t40 = t15 * t39;
        let t43 = piecewise3(t17, t18 * t24 * t34, t12 * t40 / 3.0);
        let t45 = (-param_alpha * t1 * t4 * t6 / 2.0 - t43) * t19;
        let tvrho0 = t45 * t13 / 2.0;
        vrho[ip] += tvrho0;
        // --- fxc delta (this level) (34 lines) ---
        let t47 = t20 * rho[ip];
        let t49 = 1.0 / t21 / t47;
        let t54 = t18 * t19;
        let t55 = t33 * t33;
        let t56 = 1.0 / t55;
        let t57 = t23 * t56;
        let t59 = 1.0 / t13 / t20;
        let t61 = t11 * t59 * t29;
        let t64 = param_gamma * param_gamma;
        let t67 = t64 * sigma[ip] * t24 + 1.0;
        let t68 = f64::sqrt(t67);
        let t69 = 1.0 / t68;
        let t70 = t49 * param_gamma * t69;
        let t73 = -4.0 * t25 * t61 - 4.0 * t54 * t70;
        let t74 = t57 * t73;
        let t77 = t59 * t39;
        let t79 = t38 * t38;
        let t80 = 1.0 / t79;
        let t81 = t59 * t80;
        let t85 = piecewise3(t17, -8.0 / 3.0 * t18 * t19 * t49 * t34 - t54 * t74, -4.0 / 9.0 * t12 * t77 + 4.0 / 9.0 * t12 * t81);
        let t86 = t85 * t19;
        let t89 = 1.0 / t21;
        let tv2rho20 = -t86 * t13 / 2.0 + t45 * t89 / 6.0;
        v2rho2[ip] += tv2rho20;
        let t92 = param_beta * t19;
        let t95 = 1.0 / t10;
        let t96 = param_beta * t95;
        let t99 = t23 * param_gamma * t69;
        let t102 = 3.0 / 2.0 * t96 * t30 + 3.0 / 2.0 * t92 * t99;
        let t103 = t57 * t102;
        let t106 = t95 * t11;
        let t108 = t15 * t80;
        let t112 = piecewise3(t17, t92 * t23 * t34 - t54 * t103, -t106 * t108 / 6.0 + t106 * t40 / 6.0);
        let t113 = t112 * t19;
        let tv2rhosigma0 = -t113 * t13 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
    }
}
