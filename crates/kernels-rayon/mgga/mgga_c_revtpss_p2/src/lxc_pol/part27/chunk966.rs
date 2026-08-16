//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 966/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk966(t3133: f64, t73: f64, t3095: f64, t3092: f64, t2858: f64, t4786: f64, t3153: f64, t4894: f64, t3117: f64, t4900: f64, t2258: f64, t3094: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11678 = t3133 * t73;
    let t11679 = t11678 * t3095;
    let t11680 = t3092 * t11679;
    let t11683 = t2858 * t4786;
    let t11684 = t3092 * t11683;
    let t11687 = t3133 * t3153;
    let t11688 = t11687 * t4894;
    let t11689 = t3117 * t11688;
    let t11692 = t11687 * t4900;
    let t11693 = t3117 * t11692;
    let t11696 = t3094 * t2258;
    (t11678, t11680, t11684, t11687, t11689, t11693, t11696)
}
