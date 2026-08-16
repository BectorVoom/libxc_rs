//! GGA_C_OP_B88 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_b88_exc_unpol(
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
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5::<f64>(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3::<f64>(t29);
        let t31 = 1.0 / t30;
        let t32 = t23 * t31;
        let t33 = t23 * t23;
        let t34 = sigma[ip] * t33;
        let t35 = rho[ip] * rho[ip];
        let t36 = pow_1_3::<f64>(rho[ip]);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = f64::sqrt(sigma[ip]);
        let t41 = t40 * t23;
        let t43 = 1.0 / t36 / rho[ip];
        let t45 = f64::ln(t41 * t43 + f64::sqrt(pow_2::<f64>(t41 * t43) + 1.0));
        let t46 = t43 * t45;
        let t49 = 1.0 + 0.252e-1 * t41 * t46;
        let t50 = 1.0 / t49;
        let t55 = 1.0 + 0.93333333333333333332e-3 * t22 * t34 * t39 * t50;
        let t56 = 1.0 / t55;
        let t60 = piecewise3::<f64>(t14, 0.0, t22 * t32 * t56 / 9.0);
        let t64 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t65 = piecewise5::<f64>(t26, t5, t24, t6, -t7);
        let t66 = 1.0 + t65;
        let t67 = t66 * rho[ip];
        let t68 = pow_1_3::<f64>(t67);
        let t69 = 1.0 / t68;
        let t70 = t23 * t69;
        let t74 = piecewise3::<f64>(t64, 0.0, t22 * t70 * t56 / 9.0);
        let t75 = t60 + t74;
        let t76 = t75 == 0.0;
        let t77 = piecewise3::<f64>(t76, f64::EPSILON, t75);
        let t80 = 0.36011538e1 / t77 + 0.5764e0;
        let t81 = t77 * t77;
        let t82 = t81 * t81;
        let t83 = 1.0 / t82;
        let t85 = t81 * t77;
        let t86 = 1.0 / t85;
        let t88 = 1.0 / t81;
        let t90 = 0.31390124030721e2 * t83 + 0.149643497914092e2 * t86 + 0.17833359087e1 * t88;
        let t91 = 1.0 / t90;
        let tzk0 = piecewise3::<f64>(t4, 0.0, -0.25e0 * t10 * t80 * t91);
        zk[ip] += tzk0;
    }
}
