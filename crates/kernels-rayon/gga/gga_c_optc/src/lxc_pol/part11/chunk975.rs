//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 975/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk975(t17780: f64, t17851: f64, t241: f64, t17429: f64, t17431: f64, t17433: f64, t17435: f64, t17438: f64, t17527: f64, t17645: f64, t17655: f64, t17658: f64, t17750: f64, t17753: f64) -> (f64, f64) {
    let t17853 = t241 * (t17780 + t17851);
    let t17854 = t17429 + t17431 + t17433 + t17435 - t17438 - t17527 + t17750 - t17655 + t17658 - t17753 - t17645 + t17853;
    (t17853, t17854)
}
