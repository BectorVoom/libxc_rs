//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 579/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk579(t7646: f64, t22: f64, t3826: f64, t7635: f64, t2115: f64, t7638: f64, t3819: f64, t7642: f64, t2118: f64, t7645: f64, t3851: f64, t7199: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7647 = 0.24244143692662525982e-1_f64 * t7646;
    let t7648 = t3826 * t22;
    let t7649 = t7648 * t7635;
    let t7651 = t2115 * t7638;
    let t7652 = 0.4838420607177634088e-3_f64 * t7651;
    let t7653 = t3819 * t22;
    let t7654 = t7653 * t7642;
    let t7656 = t2118 * t7645;
    let t7658 = t3851 * t7199;
    (t7647, t7648, t7649, t7651, t7652, t7653, t7654, t7656, t7658)
}
