//! MGGA_X_MSB exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_msb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_msb_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
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
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t36 = 5.0 / 972.0 * t26 * t34;
        let t37 = param_kappa + t36;
        let t41 = param_kappa * (1.0 - param_kappa / t37);
        let t42 = tau[ip] * t28;
        let t44 = 1.0 / t31 / rho[ip];
        let t45 = t42 * t44;
        let t47 = t45 - t34 / 8.0;
        let t48 = t47 * t47;
        let t49 = t21 * t21;
        let t52 = t45 + 3.0 / 10.0 * t49 * t24;
        let t53 = t52 * t52;
        let t54 = 1.0 / t53;
        let t57 = -4.0 * t48 * t54 + 1.0;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = t48 * t47;
        let t61 = t53 * t52;
        let t62 = 1.0 / t61;
        let t65 = t48 * t48;
        let t67 = param_b * t65 * t48;
        let t68 = t53 * t53;
        let t70 = 1.0 / t68 / t53;
        let t73 = 8.0 * t60 * t62 + 64.0 * t67 * t70 + 1.0;
        let t74 = 1.0 / t73;
        let t75 = t59 * t74;
        let t76 = param_kappa + t36 + param_c;
        let t81 = param_kappa * (1.0 - param_kappa / t76) - t41;
        let t83 = t75 * t81 + t41 + 1.0;
        let t87 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t83);
        let tzk0 = 2.0 * t87;
        zk[ip] += tzk0;
    }
}
