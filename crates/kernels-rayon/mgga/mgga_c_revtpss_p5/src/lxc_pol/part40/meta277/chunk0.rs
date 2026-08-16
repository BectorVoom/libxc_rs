//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1019/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1019(t1386: f64, t2681: f64, t820: f64, t1401: f64, t4000: f64, t843: f64, t4006: f64, t136: f64, t4011: f64, t221: f64, t3829: f64, t3978: f64) -> (f64, f64, f64, f64, f64) {
    let t9909 = t820 * t1386 * t2681;
    let t9910 = t9909 * t1401;
    let t9918 = t820 * t4000 * t843;
    let t9919 = t9918 * t4006;
    let t9921 = t4011 * t136;
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    (t9909, t9910, t9919, t9921, t9924)
}
