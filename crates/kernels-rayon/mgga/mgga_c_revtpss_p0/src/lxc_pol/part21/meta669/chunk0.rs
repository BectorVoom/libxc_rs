//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2470/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2470(t1086: f64, t3259: f64, t994: f64, t3046: f64, t4980: f64, t12153: f64, t12046: f64, t989: f64, t1035: f64, t42859: f64, t342: f64, t11902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43357 = t994 * t1086 * t3259;
    let t43360 = t3046 * t4980;
    let t43378 = t3046 * t12153;
    let t43384 = t989 * t12046;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43413 = t11902 * t1086;
    (t43357, t43360, t43378, t43384, t43400, t43401, t43413)
}
