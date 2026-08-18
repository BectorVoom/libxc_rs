//! GGA_X_HERMAN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_herman.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_herman_exc_pol(
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
        let t5 = t2 / t3;
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
        let t27 = t25 * t26;
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = 1.0 + 0.0006666666666666666 * t32 * t34 * t39;
        let t47 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t16;
        let t51 = piecewise5(t14, t11, t10, t15, t49 * t7);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t22, t54 * t52);
        let t57 = t56 * t26;
        let t58 = t33 * sigma2;
        let t59 = rho1 * rho1;
        let t60 = pow_1_3(rho1);
        let t61 = t60 * t60;
        let t63 = 1.0 / t61 / t59;
        let t67 = 1.0 + 0.0006666666666666666 * t32 * t58 * t63;
        let t71 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t57 * t67);
        let tzk0 = t47 + t71;
        zk[ip] += tzk0;
    }
}
