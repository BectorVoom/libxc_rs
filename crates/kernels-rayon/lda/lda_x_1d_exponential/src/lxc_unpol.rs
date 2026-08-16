//! LDA_X_1D_EXPONENTIAL lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_exponential.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::integrate::{xc_integrate_lda_exponential_func1, xc_integrate_lda_exponential_func2};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};

/// LDA_X_1D_EXPONENTIAL lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_1d_exponential_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t3, t5, t3, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t11 = t8 * M_PI * param_beta * rho[ip];
        let t12 = xc_integrate_lda_exponential_func1(t11);
        let t14 = xc_integrate_lda_exponential_func2(t11);
        let t16 = t14 / M_PI;
        let t17 = 1.0 / param_beta;
        let t18 = 1.0 / rho[ip];
        let t24 = piecewise3(t4, 0.0, -0.07957747154594767 * (-t16 * t17 * t18 + t8 * t12) * t17);
        let tzk0 = 2.0 * t24;
        zk[ip] += tzk0;
        let t25 = param_beta * param_beta;
        let t26 = 1.0 / t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = 1.0 / t27;
        let t29 = t26 * t28;
        let t32 = piecewise3(t4, 0.0, -0.07957747154594767 * t16 * t29);
        let tvrho0 = 2.0 * rho[ip] * t32 + 2.0 * t24;
        vrho[ip] += tvrho0;
        let t36 = t8 * t8;
        let t37 = t36 * M_PI;
        let t38 = M_PI * M_PI;
        let t42 = xc_e1_scaled(t36 * t38 * t25 * t27);
        let t47 = 1.0 / t27 / rho[ip];
        let t48 = t26 * t47;
        let t52 = piecewise3(t4, 0.0, -0.07957747154594767 * t37 * t42 * t18 + 0.15915494309189535 * t16 * t48);
        let tv2rho20 = 2.0 * rho[ip] * t52 + 4.0 * t32;
        v2rho2[ip] += tv2rho20;
        let t56 = t36 * t36;
        let t58 = t56 * t38 * M_PI;
        let t61 = 1.0 / t36 / t38;
        let t63 = -t61 * t29 + t42;
        let t64 = t63 * t25;
        let t70 = t27 * t27;
        let t76 = piecewise3(t4, 0.0, -0.15915494309189535 * t58 * t64 + 0.238732414637843 * t37 * t42 * t28 - 0.477464829275686 * t16 * t26 / t70);
        let tv3rho30 = 2.0 * rho[ip] * t76 + 6.0 * t52;
        v3rho3[ip] += tv3rho30;
        let t102 = piecewise3(t4, 0.0, -0.15915494309189535 * t58 * (2.0 * t63 * t36 * t38 * t25 * rho[ip] + 2.0 * t61 * t48) * t25 + 0.477464829275686 * t58 * t64 * t18 - 0.954929658551372 * t37 * t42 * t47 + 1.909859317102744 * t16 * t26 / t70 / rho[ip]);
        let tv4rho40 = 2.0 * rho[ip] * t102 + 8.0 * t76;
        v4rho4[ip] += tv4rho40;
    }
}
