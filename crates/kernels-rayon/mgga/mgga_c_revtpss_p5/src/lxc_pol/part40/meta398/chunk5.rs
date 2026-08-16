//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1451/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1451(t17807: f64, t489: f64, t3759: f64, t5230: f64, t1811: f64, t3601: f64, t3769: f64, t16695: f64, t17454: f64, t473: f64, t5412: f64, t1214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17808 = t489 * t17807;
    let t17811 = t3759 * t5230;
    let t17814 = t1811 * t3601;
    let t17815 = t17814 * t3769;
    let t17818 = t16695 * t17454;
    let t17821 = t473 * t5412;
    let t17822 = t17821 * t1214;
    (t17808, t17811, t17814, t17815, t17818, t17822)
}
