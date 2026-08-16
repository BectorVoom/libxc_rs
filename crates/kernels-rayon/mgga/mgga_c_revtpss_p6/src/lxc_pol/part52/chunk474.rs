//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 474/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk474(t360: f64, t365: f64, t1038: f64, t72: f64, t1087: f64, t1066: f64, t828: f64, t1043: f64, t73: f64, t357: f64, t905: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3088 = t360 * t365;
    let t3089 = t1038 * t72;
    let t3090 = t3088 * t3089;
    let t3091 = t1087 * t3090;
    let t3092 = t828 * t1066;
    let t3093 = t1043 * t73;
    let t3094 = t357 * t905;
    let t3095 = t3094 * t606;
    (t3088, t3089, t3090, t3091, t3092, t3093, t3095)
}
