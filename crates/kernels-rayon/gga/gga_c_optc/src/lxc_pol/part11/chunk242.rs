//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 242/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk242(t228: f64, t216: f64, t217: f64, t765: f64, t214: f64, t136: f64, t529: f64, t222: f64, t224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t777 = t228 * t228;
    let t778 = 1.0_f64 / t777;
    let t779 = t216 * t778;
    let t780 = 1.0_f64 / t217;
    let t785 = 0.29896666666666666667e0_f64 * t765;
    let t787 = f64::sqrt(t214);
    let t790 = t529 * t136;
    let t792 = t222 * t790 * t224;
    (t777, t778, t779, t780, t785, t787, t790, t792)
}
