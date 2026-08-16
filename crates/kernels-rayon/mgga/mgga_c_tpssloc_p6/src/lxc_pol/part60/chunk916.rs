//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 916/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk916(t225: f64, t28051: f64, t2006: f64, t6387: f64, t6414: f64, t1824: f64, t7722: f64, t214: f64, t6434: f64, t28108: f64, t1808: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96913 = t28051 * t225;
    let t97172 = t2006 * t6387;
    let t97181 = t2006 * t6414;
    let t97189 = t7722 * t1824;
    let t97511 = t214 * t6434;
    let t97558 = t28108 * t225;
    let t97626 = t1808 * t254;
    (t96913, t97172, t97181, t97189, t97511, t97558, t97626)
}
