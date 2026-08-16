//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2632/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2632(t2782: f64, t2797: f64, t62637: f64, t18615: f64, t251: f64, t231: f64, t2783: f64, t10069: f64, t18738: f64, t18742: f64, t10073: f64, t10530: f64, t18718: f64, t2470: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62639 = t2782 * t2797 * t62637;
    let t62641 = t251 * t18615;
    let t62644 = t2782 * t2783 * t62641 * t231;
    let t62649 = t10069 * t18738;
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62665 = t10530 * t18718 * t2470;
    (t62639, t62641, t62644, t62649, t62651, t62653, t62665)
}
