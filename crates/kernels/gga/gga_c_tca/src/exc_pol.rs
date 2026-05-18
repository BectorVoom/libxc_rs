//! GGA_C_TCA exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_tca.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_tca_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
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
        let t32 = 0.488827e1 + 0.79425925e0 * t24 * t26 / t27;
        let t33 = f64::atan(t32);
        let t35 = -0.655868e0 * t33 + 0.897889e0;
        let t36 = t20 * t35;
        let t37 = t21 * t21;
        let t38 = t36 * t37;
        let t39 = 1.0 / t23;
        let t40 = t39 * t25;
        let t41 = M_CBRT6;
        let t42 = t41 * t41;
        let t43 = M_PI * M_PI;
        let t44 = pow_1_3::<f64>(t43);
        let t45 = 1.0 / t44;
        let t46 = t42 * t45;
        let t47 = M_CBRT2;
        let t49 = sigma0 + 2.0 * sigma1 + sigma2;
        let t50 = f64::sqrt(t49);
        let t51 = t47 * t50;
        let t52 = t27 * t2;
        let t53 = 1.0 / t52;
        let t55 = t46 * t51 * t53;
        let t56 = f64::powf(t55, 0.23e1);
        let t58 = 1.0 + 0.47121507034422759993e-2 * t56;
        let t59 = 1.0 / t58;
        let t62 = t38 * t40 * t27 * t59;
        let tzk0 = t62 / 3.0;
        zk[ip] += tzk0;
    }
}
