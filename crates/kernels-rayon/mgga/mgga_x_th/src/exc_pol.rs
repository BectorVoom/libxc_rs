//! MGGA_X_TH exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_th_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let t3 = M_CBRTPI;
        let t4 = t3 * t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3(t18);
        let t24 = piecewise3(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3(t5);
        let t27 = 1.0 / tau0;
        let t28 = t26 * t27;
        let t29 = t25 * t28;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t37 = 1.0 + 7.0 / 216.0 * sigma0 / rho0 * t27;
        let t40 = pow_1_3(1.0 / M_PI);
        let t42 = M_CBRT4;
        let t43 = 1.0 / t40 * t42;
        let t44 = t31 * rho0 * t37 * t43;
        let t47 = piecewise3(t2, 0.0, -27.0 / 80.0 * t29 * t44);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t15;
        let t51 = piecewise5(t13, t10, t9, t14, t49 * t6);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t21, t54 * t52);
        let t57 = t4 * t56;
        let t58 = 1.0 / tau1;
        let t59 = t26 * t58;
        let t60 = t57 * t59;
        let t61 = pow_1_3(rho1);
        let t62 = t61 * t61;
        let t68 = 1.0 + 7.0 / 216.0 * sigma2 / rho1 * t58;
        let t70 = t62 * rho1 * t68 * t43;
        let t73 = piecewise3(t48, 0.0, -27.0 / 80.0 * t60 * t70);
        let tzk0 = t47 + t73;
        zk[ip] += tzk0;
    }
}
