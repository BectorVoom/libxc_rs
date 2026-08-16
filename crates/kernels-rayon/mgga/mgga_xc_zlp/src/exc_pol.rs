//! MGGA_XC_ZLP exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_zlp_exc_pol(
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
        let t2 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t11 = sigma0 + 2.0 * sigma1 + sigma2;
        let t12 = rho0 + rho1;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t12);
        let t15 = t14 * t14;
        let t17 = 1.0 / t15 / t13;
        let t19 = pow_1_3(rho0);
        let t20 = t19 * t19;
        let t22 = 1.0 / t20 / rho0;
        let t23 = lapl0 * t22;
        let t24 = rho0 - rho1;
        let t25 = 1.0 / t12;
        let t26 = t24 * t25;
        let t28 = 1.0 / 2.0 + t26 / 2.0;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = t30 * t28;
        let t33 = pow_1_3(rho1);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho1;
        let t37 = lapl1 * t36;
        let t39 = 1.0 / 2.0 - t26 / 2.0;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = t41 * t39;
        let t49 = 0.207108e0 * t5 * t7 + 0.5387725e-2 * t5 * t7 * (t11 * t17 / 8.0 - t23 * t31 / 8.0 - t37 * t42 / 8.0);
        let t52 = 1.0 + 0.48849425066691677572e3 / t14;
        let t53 = f64::ln(t52);
        let t56 = 1.0 - 0.2047107e-2 * t53 * t14;
        let t58 = t2 * t2;
        let t59 = t49 * t56 * t58;
        let t60 = 1.0 / t4;
        let t61 = t60 * t6;
        let t62 = t61 * t14;
        let t63 = t59 * t62;
        let tzk0 = -t63 / 3.0;
        zk[ip] += tzk0;
    }
}
