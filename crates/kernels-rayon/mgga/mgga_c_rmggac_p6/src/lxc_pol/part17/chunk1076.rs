//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1076/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1076(t1910: f64, t236: f64, t495: f64, t7230: f64, t7231: f64, t3351: f64, t498: f64, t9210: f64, t321: f64, t7248: f64, t333: f64, t511: f64) -> (f64, f64, f64, f64) {
    let t47629 = t7230 * t7231 * t236 * t1910 * t495;
    let t47634 = t3351 * t9210 * t236 * t1910 * t498;
    let t47639 = t3351 * t7248 * t236 * t1910 * t321;
    let t47644 = t3351 * t7231 * t511 * t1910 * t333;
    (t47629, t47634, t47639, t47644)
}
