//! GGA_C_CCDF vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT6, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ccdf_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t3 = 1.0 / t2;
        let t5 = param_c2 * t3 + 1.0;
        let t6 = 1.0 / t5;
        let t7 = param_c1 * t6;
        let t8 = M_CBRT2;
        let t9 = M_CBRT6;
        let t10 = t9 * t9;
        let t11 = t8 * t10;
        let t12 = M_PI * M_PI;
        let t13 = pow_1_3(t12);
        let t14 = 1.0 / t13;
        let t16 = sigma0 + 2.0 * sigma1 + sigma2;
        let t17 = f64::sqrt(t16);
        let t18 = t14 * t17;
        let t20 = 1.0 / t2 / t1;
        let t26 = f64::exp(-param_c4 * (t11 * t18 * t20 / 12.0 - param_c5));
        let t27 = 1.0 + t26;
        let t30 = 1.0 - param_c3 / t27;
        let tzk0 = t7 * t30;
        zk[ip] += tzk0;
        let t31 = t3 * param_c1;
        let t32 = t5 * t5;
        let t33 = 1.0 / t32;
        let t39 = t6 * param_c3;
        let t40 = t27 * t27;
        let t41 = 1.0 / t40;
        let t42 = t39 * t41;
        let t43 = t20 * param_c1 * t42;
        let t45 = param_c4 * t8 * t10;
        let tvrho0 = tzk0 + t31 * t33 * t30 * param_c2 / 3.0 + t43 * t45 * t18 * t26 / 9.0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t50 = t31 * t42;
        let t51 = 1.0 / t17;
        let t54 = t45 * t14 * t51 * t26;
        let t55 = t50 * t54;
        let tvsigma0 = -t55 / 24.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -t55 / 12.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
