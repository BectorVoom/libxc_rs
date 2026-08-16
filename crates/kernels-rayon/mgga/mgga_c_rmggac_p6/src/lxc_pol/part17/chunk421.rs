//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 421/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk421(t142: f64, t265: f64, t6: f64, t4130: f64, t4133: f64, t4136: f64, t4138: f64, t4142: f64, t4144: f64, t4146: f64, t410: f64, t417: f64) -> (f64, f64, f64) {
    let t4149 = t142 * t6 * t265;
    let t4151 = -0.34523333333333333333e1_f64 * t4130 + 0.23015555555555555556e1_f64 * t4133 - 0.26851481481481481482e1_f64 * t4136 - 0.93932222222222222223e0_f64 * t4138 + 0.73355e-1_f64 * t4142 - 0.14671e0_f64 * t4144 - 0.17116166666666666667e0_f64 * t4146 - 0.36793333333333333333e0_f64 * t4149;
    let t4153 = t410 * t4151 * t417;
    (t4149, t4151, t4153)
}
