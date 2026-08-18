//! GGA_X_G96 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_g96_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT4;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t38 = t34 * t37;
        let t39 = f64::sqrt(t38);
        let t40 = t39 * t38;
        let t44 = 1.0 + 2.0 / 1233.0 * t32 * t33 * t40;
        let t48 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t44);
        let t49 = rho1 <= dens_threshold;
        let t50 = -t16;
        let t52 = piecewise5(t14, t11, t10, t15, t50 * t7);
        let t53 = 1.0 + t52;
        let t54 = t53 <= zeta_threshold;
        let t55 = pow_1_3(t53);
        let t57 = piecewise3(t54, t22, t55 * t53);
        let t59 = f64::sqrt(sigma2);
        let t60 = pow_1_3(rho1);
        let t62 = 1.0 / t60 / rho1;
        let t63 = t59 * t62;
        let t64 = f64::sqrt(t63);
        let t65 = t64 * t63;
        let t69 = 1.0 + 2.0 / 1233.0 * t32 * t33 * t65;
        let t73 = piecewise3(t49, 0.0, -3.0 / 8.0 * t5 * t57 * t26 * t69);
        let tzk0 = t48 + t73;
        zk[ip] += tzk0;
    }
}
