//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1294/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1294(t19691: f64, t4801: f64, t1042: f64, t140: f64, t6284: f64, t1011: f64, t6288: f64, t6292: f64, t1015: f64, t18281: f64, t1012: f64, t3172: f64, t6262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19894 = t4801 * t19691;
    let t19895 = t1042 * t19894;
    let t19900 = t140 * t6284;
    let t19901 = t1011 * t19900;
    let t19907 = t140 * t6288;
    let t19908 = t1011 * t19907;
    let t19912 = t140 * t6292;
    let t19913 = t1011 * t19912;
    let t19916 = t1015 * t18281;
    let t19917 = t1012 * t19916;
    let t19920 = t3172 * t6262;
    (t19895, t19901, t19908, t19913, t19917, t19920)
}
