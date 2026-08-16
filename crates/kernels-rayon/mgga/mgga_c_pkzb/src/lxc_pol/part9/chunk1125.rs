//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1125/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1125(t19377: f64, t19378: f64, t19381: f64, t19384: f64, t19387: f64, t19390: f64, t19393: f64, t19396: f64, t19397: f64, t19400: f64, t19403: f64, t19410: f64, t434: f64, t4784: f64, t4812: f64, t4820: f64, t6658: f64, t6659: f64, t6665: f64, t6679: f64, t7: f64, t974: f64, t980: f64) -> f64 {
    let t19417 = -10.0_f64 / 9.0_f64 * t19377 * t19378 + 10.0_f64 / 9.0_f64 * t19377 * t19381 - 10.0_f64 / 3.0_f64 * t6679 * t19384 - 10.0_f64 * t6658 * t19387 + 10.0_f64 * t6679 * t19390 - 160.0_f64 / 9.0_f64 * t19393 * t6659 - 10.0_f64 / 9.0_f64 * t19396 * t19397 - 10.0_f64 / 9.0_f64 * t19396 * t19400 + 10.0_f64 / 3.0_f64 * t6658 * t19403 - 6160.0_f64 / 81.0_f64 * t4784 * t974 - 40.0_f64 / 3.0_f64 * t434 * t6665 - 10.0_f64 * t7 * t19410 - 40.0_f64 / 9.0_f64 * t980 * t4820 + 80.0_f64 / 81.0_f64 * t980 * t4812;
    t19417
}
