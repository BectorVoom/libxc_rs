//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1107/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1107(t90: f64, t29: f64, t560: f64, t9655: f64, t4146: f64, t550: f64, t9794: f64, t243: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t49068 = t9794 * t550;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    (t45972, t46361, t47672, t49068, t51076, t60221, t60224)
}
