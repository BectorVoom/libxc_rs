//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1169/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1169(t10415: f64, t10418: f64, t10423: f64, t10428: f64, t19396: f64, t2500: f64, t28649: f64, t28653: f64, t28659: f64, t28662: f64, t28665: f64, t28671: f64, t28677: f64, t3324: f64, t434: f64, t445: f64, t6658: f64, t7: f64) -> f64 {
    let t28792 = 80.0_f64 / 81.0_f64 * t434 * t10415 + 40.0_f64 / 81.0_f64 * t7 * t28649 - 10.0_f64 / 9.0_f64 * t19396 * t28653 - 80.0_f64 / 9.0_f64 * t434 * t10418 - 10.0_f64 / 9.0_f64 * t19396 * t28659 + 10.0_f64 / 3.0_f64 * t6658 * t28662 + 10.0_f64 / 3.0_f64 * t7 * t28665 - 40.0_f64 / 9.0_f64 * t434 * t10423 + 10.0_f64 / 9.0_f64 * t7 * t28671 + 5.0_f64 / 3.0_f64 * t7 * t28677 - 6160.0_f64 / 81.0_f64 * t10428 * t445 + 880.0_f64 / 27.0_f64 * t3324 * t2500;
    t28792
}
