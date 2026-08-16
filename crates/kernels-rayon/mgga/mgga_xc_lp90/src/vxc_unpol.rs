//! MGGA_XC_LP90 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_lp90_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569e0 + 0.37655e-3 * sigma[ip] * t6 - 0.37655e-3 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.40743e-2;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
        let t18 = t2 * rho[ip];
        let t20 = 1.0 / t4 / t18;
        let t25 = -0.10041333333333333333e-2 * sigma[ip] * t20 + 0.62758333333333333333e-3 * lapl[ip] * t6;
        let t29 = t15 * t15;
        let t30 = 1.0 / t29;
        let tvrho0 = tzk0 - rho[ip] * t25 * t16 - t14 * t13 * t30 / 3.0;
        vrho[ip] += tvrho0;
        let t33 = t10 * t16;
        let tvsigma0 = -0.37655e-3 * t33;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.37655e-3 / t4 * t16;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
