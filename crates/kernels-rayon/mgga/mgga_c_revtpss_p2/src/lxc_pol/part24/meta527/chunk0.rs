//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1560/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1560(t1032: f64, t1246: f64, t24698: f64, t1222: f64, t140: f64, t24830: f64, t17471: f64, t24236: f64, t24679: f64, t369: f64, t467: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t83607 = t24698 * t1032 * t1246;
    let t83699 = t1222 * t140 * t24830;
    let t83719 = t1222 * t17471 * t24236;
    let t83725 = t467 * t475 * t24679 * t369;
    (t83607, t83699, t83719, t83725)
}
