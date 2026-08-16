//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1275/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1275(t1678: f64, t3316: f64, t342: f64, t6299: f64, t73: f64, t4976: f64, t1082: f64, t19414: f64, t1045: f64, t999: f64, t6271: f64, t3117: f64) -> (f64, f64, f64, f64, f64) {
    let t19607 = t3316 * t1678;
    let t19608 = t342 * t19607;
    let t19611 = t6299 * t73;
    let t19612 = t19611 * t4976;
    let t19617 = t1082 * t19414;
    let t19620 = t1045 * t999;
    let t19621 = t6271 * t19620;
    let t19622 = t3117 * t19621;
    (t19608, t19611, t19612, t19617, t19622)
}
