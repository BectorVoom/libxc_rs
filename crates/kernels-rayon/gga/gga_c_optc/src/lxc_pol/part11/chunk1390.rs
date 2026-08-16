//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1390/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1390(t58701: f64, t58714: f64, t58756: f64, t58782: f64, t1085: f64, t1094: f64, t1102: f64, t1220: f64, t1221: f64, t15008: f64, t15012: f64, t3284: f64, t43809: f64, t43834: f64, t43865: f64, t5103: f64, t53470: f64, t53494: f64, t58386: f64, t58390: f64, t58403: f64, t8426: f64, t914: f64) -> (f64, f64, f64) {
    let t58784 = t58701 + t58714 + t58756 + t58782;
    let t58788 = 0.58482233974552040708e0_f64 * t1102 * t1085 * t58784 * t1094;
    let t58791 = -64.0_f64 / 9.0_f64 * t15008 * t5103 + 4.0_f64 / 3.0_f64 * t15012 * t5103 - 1520000.0_f64 / 243.0_f64 * t53470 - 400.0_f64 / 81.0_f64 * t43809 - t1220 * t914 * t1221 * t58390 + 2.0_f64 / 3.0_f64 * t1220 * t914 * t3284 * t58403 - 56.0_f64 / 9.0_f64 * t1220 * t914 * t8426 * t58386 - 2.0_f64 / 3.0_f64 * t43834 - t58788 + 56.0_f64 / 81.0_f64 * t53494 + 32.0_f64 / 27.0_f64 * t43865;
    (t58784, t58788, t58791)
}
