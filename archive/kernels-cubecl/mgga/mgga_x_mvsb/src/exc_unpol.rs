//! MGGA_X_MVSB exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mvsb_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t7 * t18;
        let t20 = pow_1_3::<f64>(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t20 * t20;
        let t26 = 1.0 / t24 / rho[ip];
        let t27 = t23 * t26;
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t34 = t27 - t28 * t31 / 8.0;
        let t35 = M_CBRT6;
        let t36 = t35 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3::<f64>(t37);
        let t39 = t38 * t38;
        let t42 = t27 - 3.0 / 10.0 * t36 * t39;
        let t43 = 1.0 / t42;
        let t46 = param_k0 * (-t34 * t43 + 1.0);
        let t47 = t34 * t34;
        let t48 = param_e1 * t47;
        let t49 = t42 * t42;
        let t50 = 1.0 / t49;
        let t52 = t48 * t50 + 1.0;
        let t53 = t52 * t52;
        let t54 = t47 * t47;
        let t55 = param_c1 * t54;
        let t56 = t49 * t49;
        let t57 = 1.0 / t56;
        let t59 = t55 * t57 + t53;
        let t60 = pow_1_4::<f64>(t59);
        let t61 = 1.0 / t60;
        let t63 = t46 * t61 + 1.0;
        let t67 = 1.0 / t38 / t37;
        let t69 = sigma[ip] * sigma[ip];
        let t71 = t29 * t29;
        let t72 = t71 * rho[ip];
        let t74 = 1.0 / t20 / t72;
        let t78 = 1.0 + param_b * t36 * t67 * t69 * t21 * t74 / 288.0;
        let t79 = f64::powf(t78, 1.0 / 8.0);
        let t80 = 1.0 / t79;
        let t84 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t63 * t80);
        let tzk0 = 2.0 * t84;
        zk[ip] += tzk0;
    }
}
