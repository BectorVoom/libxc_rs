//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2633/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2633(t18719: f64, t39609: f64, t18761: f64, t2470: f64, t874: f64, t14602: f64, t2482: f64, t2811: f64, t5977: f64, t2801: f64, t879: f64, t10073: f64, t18750: f64) -> (f64, f64, f64, f64, f64) {
    let t62667 = t39609 * t18719;
    let t62670 = t874 * t18761 * t2470;
    let t62675 = t2482 * t2811 * t5977 * t14602;
    let t62682 = t2482 * t879 * t5977 * t2801;
    let t62684 = t10073 * t18750;
    (t62667, t62670, t62675, t62682, t62684)
}
