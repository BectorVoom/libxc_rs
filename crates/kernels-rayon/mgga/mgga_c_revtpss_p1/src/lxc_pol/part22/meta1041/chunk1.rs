//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3635/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3635(t45046: f64, t6474: f64, t3383: f64, t6433: f64, t3386: f64, t5180: f64, t1188: f64, t1196: f64, t3495: f64, t16811: f64, t43752: f64, t6518: f64) -> (f64, f64, f64, f64, f64) {
    let t68791 = 0.16081979498692535067e2_f64 * t45046 * t6474;
    let t68792 = t6433 * t3383;
    let t68794 = 2.0_f64 * t68792 * t3386;
    let t68795 = t5180 * t5180;
    let t68799 = 0.23392894490538584828e1_f64 * t1196 * t3495 * t68795 * t1188;
    let t68803 = 0.12304822629859687989e5_f64 * t1196 * t43752 * t6518 * t16811;
    (t68791, t68794, t68795, t68799, t68803)
}
