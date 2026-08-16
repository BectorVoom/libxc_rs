//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1052/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1052(t10100: f64, t236: f64, t321: f64, t7230: f64, t9188: f64, t333: f64, t3352: f64, t511: f64, t352: f64, t515: f64, t1986: f64, t305: f64, t552: f64, t615: f64) -> (f64, f64, f64, f64) {
    let t47903 = t7230 * t9188 * t236 * t10100 * t321;
    let t47908 = t7230 * t3352 * t511 * t10100 * t333;
    let t47913 = t7230 * t3352 * t515 * t10100 * t352;
    let t47917 = t1986 * t305 * t552 * t615;
    (t47903, t47908, t47913, t47917)
}
