//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2762/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2762(t3938: f64, t73856: f64, t9816: f64, t9818: f64, t1412: f64, t6843: f64, t2661: f64, t3992: f64, t1399: f64, t22020: f64, t46766: f64, t6864: f64) -> (f64, f64, f64, f64, f64) {
    let t73859 = t9816 * t9818 * t73856 * t3938;
    let t73920 = t1412 * t6843;
    let t73923 = t2661 * t3992 * t73920 * t3938;
    let t73927 = t2661 * t3992 * t22020 * t1399;
    let t73929 = t46766 * t6864;
    (t73859, t73920, t73923, t73927, t73929)
}
