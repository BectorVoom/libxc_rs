//! GGA_C_OP_XALPHA exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_xalpha_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5::<f64>(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3::<f64>(1.0 / M_PI);
        let t20 = t16 / t18;
        let t21 = M_CBRT4;
        let t22 = M_CBRT2;
        let t23 = t21 * t22;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5::<f64>(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3::<f64>(t29);
        let t35 = piecewise3::<f64>(t14, 0.0, t20 * t23 / t30 / 9.0);
        let t39 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t40 = piecewise5::<f64>(t26, t5, t24, t6, -t7);
        let t41 = 1.0 + t40;
        let t42 = t41 * rho[ip];
        let t43 = pow_1_3::<f64>(t42);
        let t48 = piecewise3::<f64>(t39, 0.0, t20 * t23 / t43 / 9.0);
        let t49 = t35 + t48;
        let t50 = t49 == 0.0;
        let t51 = piecewise3::<f64>(t50, f64::EPSILON, t49);
        let t54 = 0.390299956e1 / t51 + 0.5764e0;
        let t55 = t51 * t51;
        let t56 = t55 * t55;
        let t57 = 1.0 / t56;
        let t59 = t55 * t51;
        let t60 = 1.0 / t59;
        let t62 = 1.0 / t55;
        let t64 = 0.433132090567376656e2 * t57 + 0.190514637481962976e2 * t60 + 0.2094820520028e1 * t62;
        let t65 = 1.0 / t64;
        let tzk0 = piecewise3::<f64>(t4, 0.0, -0.25e0 * t10 * t54 * t65);
        zk[ip] += tzk0;
    }
}
