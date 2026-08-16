//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2660/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660(t20112: f64, t994: f64, t4746: f64, t4930: f64, t19855: f64, t993: f64, t378: f64, t15654: f64, t1678: f64, t225: f64, t11249: f64, t6299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64737 = t994 * t20112;
    let t64764 = t4746 * t4930;
    let t64816 = t19855 * t993;
    let t64817 = t64816 * t378;
    let t64845 = t15654 * t1678;
    let t64907 = t64816 * t225;
    let t65144 = t6299 * t11249;
    (t64737, t64764, t64817, t64845, t64907, t65144)
}
