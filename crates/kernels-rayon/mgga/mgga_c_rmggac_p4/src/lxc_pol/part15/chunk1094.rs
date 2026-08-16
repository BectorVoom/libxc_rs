//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1094/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1094(t46428: f64, t8620: f64, t46431: f64, t8640: f64, t10100: f64, t236: f64, t498: f64, t7230: f64, t7248: f64, t321: f64, t9188: f64, t333: f64, t3352: f64, t511: f64) -> (f64, f64, f64, f64, f64) {
    let t47889 = t8620 * t46428;
    let t47891 = t8640 * t46431;
    let t47898 = t7230 * t7248 * t236 * t10100 * t498;
    let t47903 = t7230 * t9188 * t236 * t10100 * t321;
    let t47908 = t7230 * t3352 * t511 * t10100 * t333;
    (t47889, t47891, t47898, t47903, t47908)
}
