//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1450/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1450(t10069: f64, t18742: f64, t10073: f64, t18738: f64, t10530: f64, t18718: f64, t2470: f64, t18761: f64, t874: f64, t18750: f64, t136: f64, t2457: f64, t2710: f64, t6041: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62651 = t10069 * t18742;
    let t62653 = t10073 * t18738;
    let t62665 = t10530 * t18718 * t2470;
    let t62670 = t874 * t18761 * t2470;
    let t62684 = t10073 * t18750;
    let t62716 = t2710 * t6041 * t136 * t2457;
    (t62651, t62653, t62665, t62670, t62684, t62716)
}
