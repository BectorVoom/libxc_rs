//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2841/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2841(t23342: f64, t2652: f64, t221: f64, t23114: f64, t2674: f64, t40683: f64, t14648: f64, t14832: f64, t2661: f64, t5962: f64, t23346: f64, t231: f64, t2662: f64, t76569: f64) -> (f64, f64, f64, f64, f64) {
    let t76804 = t2652 * t23342;
    let t76808 = t2674 * t40683 * t221 * t23114;
    let t76812 = t2661 * t14832 * t14648 * t5962;
    let t76814 = t2652 * t23346;
    let t76818 = t2661 * t2662 * t76569 * t231;
    (t76804, t76808, t76812, t76814, t76818)
}
