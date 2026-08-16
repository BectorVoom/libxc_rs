//! GGA_X_LSRPBE exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lsrpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lsrpbe_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = 1.0 / param_kappa;
        let t45 = f64::exp(-t34 * sigma0 * t39 * t41 / 24.0);
        let t48 = param_kappa + 1.0;
        let t49 = param_alpha * t28;
        let t50 = t33 * sigma0;
        let t54 = f64::exp(-t49 * t50 * t39 / 24.0);
        let t57 = 1.0 + param_kappa * (1.0 - t45) - t48 * (1.0 - t54);
        let t61 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t16;
        let t65 = piecewise5(t14, t11, t10, t15, t63 * t7);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3(t66);
        let t70 = piecewise3(t67, t22, t68 * t66);
        let t71 = t70 * t26;
        let t72 = rho1 * rho1;
        let t73 = pow_1_3(rho1);
        let t74 = t73 * t73;
        let t76 = 1.0 / t74 / t72;
        let t81 = f64::exp(-t34 * sigma2 * t76 * t41 / 24.0);
        let t84 = t33 * sigma2;
        let t88 = f64::exp(-t49 * t84 * t76 / 24.0);
        let t91 = 1.0 + param_kappa * (1.0 - t81) - t48 * (1.0 - t88);
        let t95 = piecewise3(t62, 0.0, -3.0 / 8.0 * t5 * t71 * t91);
        let tzk0 = t61 + t95;
        zk[ip] += tzk0;
    }
}
