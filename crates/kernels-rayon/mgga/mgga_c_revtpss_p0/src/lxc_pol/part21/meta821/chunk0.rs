//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3038/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3038(t1086: f64, t4930: f64, t994: f64, t342: f64, t378: f64, t43471: f64, t3154: f64, t43350: f64, t16565: f64, t989: f64, t1071: f64, t12046: f64) -> (f64, f64, f64, f64, f64) {
    let t55934 = t994 * t1086 * t4930;
    let t55938 = t342 * t43471 * t378;
    let t55939 = t43350 * t3154;
    let t55944 = t989 * t16565;
    let t55948 = t342 * t12046 * t1071;
    (t55934, t55938, t55939, t55944, t55948)
}
