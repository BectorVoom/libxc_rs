//! GGA_K_APBEINT exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_apbeint_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_muPBE - param_muGE;
        let t25 = t24 * param_alpha;
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = t25 * t31;
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = sigma[ip] * t34;
        let t36 = rho[ip] * rho[ip];
        let t38 = 1.0 / t22 / t36;
        let t41 = t35 * t38;
        let t44 = 1.0 + param_alpha * t26 * t30 * t41 / 24.0;
        let t45 = 1.0 / t44;
        let t46 = t38 * t45;
        let t51 = (param_muGE + t32 * t35 * t46 / 24.0) * t26;
        let t52 = t51 * t30;
        let t55 = param_kappa + t52 * t41 / 24.0;
        let t60 = 1.0 + param_kappa * (1.0 - param_kappa / t55);
        let t64 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
    }
}
