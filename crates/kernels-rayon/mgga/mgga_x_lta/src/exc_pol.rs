//! MGGA_X_LTA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_lta.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_lta_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_ltafrac: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t34 = M_CBRT6;
        let t35 = M_PI * M_PI;
        let t36 = pow_1_3(t35);
        let t37 = t36 * t36;
        let t39 = t34 / t37;
        let t42 = 4.0 / 5.0 * param_ltafrac;
        let t43 = rmath::pow(5.0 / 9.0 * tau0 / t30 / rho0 * t39, t42);
        let t47 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t17;
        let t51 = piecewise5(t15, t12, t11, t16, t49 * t8);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t23, t54 * t52);
        let t57 = t56 * t27;
        let t58 = pow_1_3(rho1);
        let t59 = t58 * t58;
        let t65 = rmath::pow(5.0 / 9.0 * tau1 / t59 / rho1 * t39, t42);
        let t69 = piecewise3(t48, 0.0, -3.0 / 8.0 * t6 * t57 * t65);
        let tzk0 = t47 + t69;
        zk[ip] += tzk0;
    }
}
