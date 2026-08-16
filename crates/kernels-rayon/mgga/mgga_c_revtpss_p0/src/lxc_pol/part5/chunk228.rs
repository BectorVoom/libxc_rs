//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 228/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk228(t158: f64, t750: f64, t162: f64, t716: f64, t187: f64, t192: f64, t72: f64, t186: f64, t675: f64, t685: f64) -> (f64, f64, f64, f64, f64) {
    let t751 = t158 * t750;
    let t752 = t716 * t162;
    let t754 = 0.19751673498613801407e-1_f64 * t752 * t187;
    let t755 = t192 * t72;
    let t757 = t685 * t675 * t186;
    (t751, t752, t754, t755, t757)
}
