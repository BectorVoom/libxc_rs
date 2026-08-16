//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 490/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk490(t1145: f64, t469: f64, t454: f64, t462: f64, t442: f64) -> (f64, f64, f64, f64, f64) {
    let t3169 = 1.0_f64 / t1145 / t469;
    let t3170 = t454 * t3169;
    let t3181 = t462 * t462;
    let t3182 = 1.0_f64 / t3181;
    let t3183 = t3182 * t442;
    (t3169, t3170, t3181, t3182, t3183)
}
