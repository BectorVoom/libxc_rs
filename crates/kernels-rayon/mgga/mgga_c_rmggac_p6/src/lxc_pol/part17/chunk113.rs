//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 113/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk113(t305: f64, t321: f64, t326: f64, t333: f64, t344: f64, t349: f64) -> f64 {
    let t352 = 0.19957069503106347607e-1_f64 * t305 * t321 - 0.19957069503106347607e-1_f64 * t326 * t333 + 0.26552308210121162678e-3_f64 * t344 * t321 - 0.26552308210121162678e-3_f64 * t349 * t333;
    t352
}
