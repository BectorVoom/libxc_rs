//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 832/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk832(t1528: f64, t236: f64, t321: f64, t3351: f64, t7248: f64, t333: f64, t511: f64, t7231: f64, t352: f64, t515: f64, t7720: f64, t8582: f64) -> (f64, f64, f64, f64) {
    let t38594 = t3351 * t7248 * t236 * t1528 * t321;
    let t38599 = t3351 * t7231 * t511 * t1528 * t333;
    let t38604 = t3351 * t7231 * t515 * t1528 * t352;
    let t38606 = t7720 * t8582;
    (t38594, t38599, t38604, t38606)
}
