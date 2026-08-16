//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 703/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk703(t6693: f64, t2048: f64, t592: f64, t188: f64, t1912: f64, t1956: f64, t6647: f64, t6648: f64, t6675: f64, t6682: f64, t6684: f64, t6687: f64, t6689: f64, t737: f64) -> (f64, f64, f64) {
    let t6694 = 96.0_f64 * t6693;
    let t6695 = t2048 * t592;
    let t6696 = 96.0_f64 * t6695;
    let t6697 = -t6647 + 3.0_f64 / 2.0_f64 * t6648 + t188 * t6675 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t737 * t1956 + 35.0_f64 / 3.0_f64 * t6682 - 7.0_f64 * t6684 - 7.0_f64 / 2.0_f64 * t6687 + 3.0_f64 / 2.0_f64 * t6689 + 3.0_f64 / 2.0_f64 * t737 * t1912 - t6694 - t6696;
    (t6694, t6696, t6697)
}
