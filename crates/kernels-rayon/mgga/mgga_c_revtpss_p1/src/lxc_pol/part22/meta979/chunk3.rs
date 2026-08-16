//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3293/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3293(t14832: f64, t2661: f64, t62351: f64, t775: f64, t10716: f64, t18423: f64, t62361: f64, t14648: f64, t4343: f64, t18398: f64, t2652: f64, t18415: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62429 = t2661 * t14832 * t62351 * t775;
    let t62431 = t10716 * t18423;
    let t62435 = t2661 * t14832 * t62361 * t775;
    let t62439 = t2661 * t14832 * t14648 * t4343;
    let t62441 = t2652 * t18398;
    let t62443 = t9775 * t18415;
    (t62429, t62431, t62435, t62439, t62441, t62443)
}
