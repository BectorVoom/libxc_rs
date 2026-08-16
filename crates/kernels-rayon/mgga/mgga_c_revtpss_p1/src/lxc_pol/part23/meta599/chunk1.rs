//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2248/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2248(t24007: f64, t3155: f64, t3117: f64, t3162: f64, t11765: f64, t22688: f64, t1012: f64, t23598: f64, t373: f64, t371: f64, t372: f64, t1651: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24008 = t24007 * t3155;
    let t24009 = t3117 * t24008;
    let t24012 = t24007 * t3162;
    let t24013 = t3117 * t24012;
    let t24016 = t11765 * t22688;
    let t24017 = t1012 * t24016;
    let t24022 = t373 * t23598;
    let t24024 = t371 * t372 * t24022;
    let t24031 = t6244 * t1651;
    (t24008, t24009, t24012, t24013, t24016, t24017, t24022, t24024, t24031)
}
