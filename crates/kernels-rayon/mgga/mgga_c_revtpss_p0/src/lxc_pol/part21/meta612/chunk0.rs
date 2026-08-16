//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2364/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2364(t10573: f64, t2619: f64, t2598: f64, t9321: f64, t760: f64, t2523: f64, t9387: f64, t2495: f64, t39875: f64, t9367: f64, t10565: f64, t606: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40127 = t10573 * t2619;
    let t40129 = t9321 * t2598;
    let t40131 = 0.21053605041484726346e2_f64 * t760 * t40129;
    let t40132 = t2523 * t9387;
    let t40135 = t9367 * t39875 * t2495;
    let t40137 = 0.6233709278045326953e3_f64 * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    (t40127, t40129, t40131, t40132, t40135, t40137, t40139)
}
