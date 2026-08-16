//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1079/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1079(t90: f64, t29: f64, t560: f64, t9655: f64, t4146: f64, t550: f64, t9794: f64, t5778: f64, t9593: f64, t243: f64, t2246: f64, t4171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t49068 = t9794 * t550;
    let t49575 = t5778 * t9593;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    (t45972, t46361, t47672, t49068, t49575, t51076, t60221)
}
