//! GGA_X_N12 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_n12_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t21 = param_CC_0_1;
        let t22 = t21 * sigma[ip];
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = rho[ip] * rho[ip];
        let t26 = t18 * t18;
        let t28 = 1.0 / t26 / t25;
        let t29 = t24 * t28;
        let t33 = 1.0 + 0.004 * sigma[ip] * t24 * t28;
        let t34 = 1.0 / t33;
        let t35 = t29 * t34;
        let t38 = param_CC_0_2;
        let t39 = sigma[ip] * sigma[ip];
        let t40 = t38 * t39;
        let t41 = t25 * t25;
        let t42 = t41 * rho[ip];
        let t44 = 1.0 / t18 / t42;
        let t46 = t33 * t33;
        let t47 = 1.0 / t46;
        let t48 = t23 * t44 * t47;
        let t51 = param_CC_0_3;
        let t52 = t39 * sigma[ip];
        let t53 = t51 * t52;
        let t54 = t41 * t41;
        let t55 = 1.0 / t54;
        let t56 = t46 * t33;
        let t57 = 1.0 / t56;
        let t58 = t55 * t57;
        let t62 = param_CC_1_1;
        let t63 = t62 * sigma[ip];
        let t66 = param_CC_1_2;
        let t67 = t66 * t39;
        let t70 = param_CC_1_3;
        let t71 = t70 * t52;
        let t74 = param_CC_1_0 + 0.004 * t63 * t35 + 3.2e-05 * t67 * t48 + 2.56e-07 * t71 * t58;
        let t79 = piecewise3(t12, 1.0 / t13, 1.0 / t15);
        let t82 = 1.0 + 0.4 / t18 * t23 * t79;
        let t83 = 1.0 / t82;
        let t86 = param_CC_2_1;
        let t87 = t86 * sigma[ip];
        let t90 = param_CC_2_2;
        let t91 = t90 * t39;
        let t94 = param_CC_2_3;
        let t95 = t94 * t52;
        let t98 = param_CC_2_0 + 0.004 * t87 * t35 + 3.2e-05 * t91 * t48 + 2.56e-07 * t95 * t58;
        let t99 = t82 * t82;
        let t100 = 1.0 / t99;
        let t103 = param_CC_3_1;
        let t104 = t103 * sigma[ip];
        let t107 = param_CC_3_2;
        let t108 = t107 * t39;
        let t111 = param_CC_3_3;
        let t112 = t111 * t52;
        let t115 = param_CC_3_0 + 0.004 * t104 * t35 + 3.2e-05 * t108 * t48 + 2.56e-07 * t112 * t58;
        let t116 = t99 * t82;
        let t117 = 1.0 / t116;
        let t119 = param_CC_0_0 + 0.004 * t22 * t35 + 3.2e-05 * t40 * t48 + 2.56e-07 * t53 * t58 + t74 * t83 + t98 * t100 + t115 * t117;
        let t123 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t119);
        let tzk0 = 2.0 * t123;
        zk[ip] += tzk0;
    }
}
