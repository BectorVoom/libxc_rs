//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 964/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk964(t10990: f64, t263: f64, t6876: f64, t2315: f64, t3438: f64, t3446: f64, t1064: f64, t5086: f64) -> (f64, f64, f64, f64, f64) {
    let t10991 = 0.14905073231436680509e-2_f64 * t10990;
    let t10992 = t263 * t6876;
    let t10993 = t3438 * t2315;
    let t10995 = t3446 * t10992 * t10993;
    let t10996 = 0.30487649791575028314e-3_f64 * t10995;
    let t10997 = t5086 * t1064;
    (t10991, t10992, t10993, t10996, t10997)
}
