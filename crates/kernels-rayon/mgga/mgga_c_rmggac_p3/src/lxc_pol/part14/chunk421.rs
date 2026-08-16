//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 421/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk421(t4130: f64, t4133: f64, t4136: f64, t4138: f64, t4142: f64, t4144: f64, t4146: f64, t4149: f64, t410: f64, t417: f64, t431: f64, t1037: f64, t409: f64) -> (f64, f64, f64) {
    let t4151 = -0.34523333333333333333e1_f64 * t4130 + 0.23015555555555555556e1_f64 * t4133 - 0.26851481481481481482e1_f64 * t4136 - 0.93932222222222222223e0_f64 * t4138 + 0.73355e-1_f64 * t4142 - 0.14671e0_f64 * t4144 - 0.17116166666666666667e0_f64 * t4146 - 0.36793333333333333333e0_f64 * t4149;
    let t4153 = t410 * t4151 * t417;
    let t4155 = 0.5848223622634646207e0_f64 * t431 * t4153;
    let t4157 = 1.0_f64 / t1037 / t409;
    (t4151, t4155, t4157)
}
