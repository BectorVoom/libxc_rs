//! LDA_C_RC04 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_RC04 vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rc04_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3::<f64>(zeta_threshold);
        let t8 = t7 * t7;
        let t9 = pow_1_3::<f64>(t5);
        let t10 = t9 * t9;
        let t11 = piecewise3::<f64>(t6, t8, t10);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3::<f64>(t12);
        let t15 = t14 * t14;
        let t16 = piecewise3::<f64>(t13, t8, t15);
        let t18 = t11 / 2.0 + t16 / 2.0;
        let t19 = t18 * t18;
        let t20 = t19 * t18;
        let t21 = M_CBRT3;
        let t23 = pow_1_3::<f64>(1.0 / M_PI);
        let t24 = t21 * t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t25;
        let t27 = pow_1_3::<f64>(t2);
        let t32 = 4.88827 + 0.79425925 * t24 * t26 / t27;
        let t33 = f64::atan(t32);
        let t35 = -0.655868 * t33 + 0.897889;
        let t37 = t21 * t21;
        let t38 = t20 * t35 * t37;
        let t39 = 1.0 / t23;
        let t40 = t39 * t25;
        let t41 = t40 * t27;
        let t42 = t38 * t41;
        let tzk0 = t42 / 3.0;
        zk[ip] += tzk0;
        let t43 = 4.0 / 9.0 * t42;
        let t44 = t27 * t2;
        let t46 = t44 * t19 * t35;
        let t47 = t37 * t39;
        let t48 = 1.0 / t9;
        let t49 = t2 * t2;
        let t50 = 1.0 / t49;
        let t51 = t1 * t50;
        let t52 = t3 - t51;
        let t55 = piecewise3::<f64>(t6, 0.0, 2.0 / 3.0 * t48 * t52);
        let t56 = 1.0 / t14;
        let t57 = -t52;
        let t60 = piecewise3::<f64>(t13, 0.0, 2.0 / 3.0 * t56 * t57);
        let t62 = t55 / 2.0 + t60 / 2.0;
        let t66 = t32 * t32;
        let t67 = t66 + 1.0;
        let t68 = 1.0 / t67;
        let t70 = 0.6945723010386666 * t20 * t68;
        let tvrho0 = t46 * t47 * t25 * t62 + t43 + t70;
        vrho[ip * 2] += tvrho0;
        let t71 = -t3 - t51;
        let t74 = piecewise3::<f64>(t6, 0.0, 2.0 / 3.0 * t48 * t71);
        let t75 = -t71;
        let t78 = piecewise3::<f64>(t13, 0.0, 2.0 / 3.0 * t56 * t75);
        let t80 = t74 / 2.0 + t78 / 2.0;
        let t82 = t47 * t25 * t80;
        let tvrho1 = t46 * t82 + t43 + t70;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
