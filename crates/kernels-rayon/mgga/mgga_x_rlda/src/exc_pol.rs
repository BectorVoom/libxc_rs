//! MGGA_X_RLDA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rlda_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_prefactor: f64,
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
        let t29 = pow_1_3(1.0 / M_PI);
        let t30 = 1.0 / t29;
        let t31 = param_prefactor * t30;
        let t32 = M_CBRT4;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho0;
        let t41 = 2.0 * tau0 * t36 - lapl0 * t36 / 4.0;
        let t44 = t31 * t32 / t41;
        let t47 = piecewise3(t2, 0.0, -15.0 / 16.0 * t25 * t26 * t44);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t15;
        let t51 = piecewise5(t13, t10, t9, t14, t49 * t6);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t21, t54 * t52);
        let t57 = t4 * t56;
        let t59 = pow_1_3(rho1);
        let t60 = t59 * t59;
        let t62 = 1.0 / t60 / rho1;
        let t67 = 2.0 * tau1 * t62 - lapl1 * t62 / 4.0;
        let t70 = t31 * t32 / t67;
        let t73 = piecewise3(t48, 0.0, -15.0 / 16.0 * t57 * t26 * t70);
        let tzk0 = t47 + t73;
        zk[ip] += tzk0;
    }
}
