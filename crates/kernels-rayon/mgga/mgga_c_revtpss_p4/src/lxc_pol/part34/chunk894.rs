//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 894/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk894(t1011: f64, t19900: f64, t140: f64, t6288: f64, t6292: f64, t3172: f64, t6262: f64, t3127: f64, t1062: f64, t6317: f64, t11922: f64, t6272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19901 = t1011 * t19900;
    let t19907 = t140 * t6288;
    let t19908 = t1011 * t19907;
    let t19912 = t140 * t6292;
    let t19913 = t1011 * t19912;
    let t19920 = t3172 * t6262;
    let t19921 = t3127 * t19920;
    let t19968 = t6317 * t1062;
    let t19976 = t11922 * t6272;
    (t19901, t19907, t19908, t19912, t19913, t19920, t19921, t19968, t19976)
}
