//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 855/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk855(t321: f64, t3351: f64, t515: f64, t7248: f64, t9049: f64, t498: f64, t7230: f64, t7231: f64, t9044: f64, t3352: f64, t1986: f64, t326: f64, t495: f64, t559: f64) -> (f64, f64, f64, f64) {
    let t39127 = t3351 * t7248 * t515 * t9049 * t321;
    let t39132 = t7230 * t7231 * t515 * t9044 * t498;
    let t39137 = t7230 * t3352 * t515 * t9044 * t321;
    let t39141 = t1986 * t326 * t559 * t495;
    (t39127, t39132, t39137, t39141)
}
