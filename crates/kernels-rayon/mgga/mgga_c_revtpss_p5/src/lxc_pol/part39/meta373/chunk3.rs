//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1318/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1318(t12160: f64, t4891: f64, t1043: f64, t4772: f64, t1045: f64, t3117: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15917 = t12160 * t4891;
    let t15920 = t4772 * t1043;
    let t15921 = t15920 * t1045;
    let t15922 = t3117 * t15921;
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    let t15932 = t15822 * t3160;
    let t15935 = t1065 * t2852;
    (t15917, t15920, t15922, t15926, t15932, t15935)
}
