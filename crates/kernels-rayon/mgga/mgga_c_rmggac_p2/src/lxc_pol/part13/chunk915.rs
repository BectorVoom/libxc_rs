//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 915/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk915(t1970: f64, t1971: f64, t333: f64, t511: f64, t5605: f64, t352: f64, t515: f64, t36634: f64, t5156: f64, t656: f64, t36629: f64, t5163: f64) -> (f64, f64, f64, f64) {
    let t40177 = t1970 * t1971 * t511 * t5605 * t333;
    let t40182 = t1970 * t1971 * t515 * t5605 * t352;
    let t40185 = t36634 * t656 * t5156;
    let t40188 = t36629 * t656 * t5163;
    (t40177, t40182, t40185, t40188)
}
