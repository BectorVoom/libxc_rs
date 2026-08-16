//! GGA_X_2D_B86_MGC exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b86_mgc_exc_pol(
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
        let t1 = rho0 <= dens_threshold;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5::<f64>(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = f64::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = f64::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3::<f64>(t18, t20, t22);
        let t24 = t3 * t23;
        let t25 = M_SQRT2;
        let t26 = f64::sqrt(t4);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t31 = sigma0 * t30;
        let t33 = 1.0 + 0.8323e-2 * t31;
        let t34 = pow_1_4::<f64>(t33);
        let t35 = t34 * t34;
        let t36 = t35 * t34;
        let t37 = 1.0 / t36;
        let t40 = 1.0 + 0.22047110337950987485e-2 * t31 * t37;
        let t41 = t27 * t40;
        let t44 = piecewise3::<f64>(t1, 0.0, -2.0 / 3.0 * t24 * t41);
        let t45 = rho1 <= dens_threshold;
        let t46 = -t14;
        let t48 = piecewise5::<f64>(t12, t9, t8, t13, t46 * t5);
        let t49 = 1.0 + t48;
        let t50 = t49 <= zeta_threshold;
        let t51 = f64::sqrt(t49);
        let t52 = t51 * t49;
        let t53 = piecewise3::<f64>(t50, t20, t52);
        let t54 = t3 * t53;
        let t55 = rho1 * rho1;
        let t56 = t55 * rho1;
        let t57 = 1.0 / t56;
        let t58 = sigma2 * t57;
        let t60 = 1.0 + 0.8323e-2 * t58;
        let t61 = pow_1_4::<f64>(t60);
        let t62 = t61 * t61;
        let t63 = t62 * t61;
        let t64 = 1.0 / t63;
        let t67 = 1.0 + 0.22047110337950987485e-2 * t58 * t64;
        let t68 = t27 * t67;
        let t71 = piecewise3::<f64>(t45, 0.0, -2.0 / 3.0 * t54 * t68);
        let tzk0 = t44 + t71;
        zk[ip] += tzk0;
    }
}
