//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1087/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1087(t43: f64, t12161: f64, t1281: f64, t15072: f64, t1690: f64, t1694: f64, t19461: f64, t234: f64, t2868: f64, t2898: f64, t35: f64, t4070: f64, t5455: f64, t5481: f64, t5486: f64, t595: f64, t817: f64, t818: f64, t821: f64, t824: f64, zeta_threshold: f64) -> f64 {
    let t44 = t43 <= zeta_threshold;
    let t19482 = piecewise3(t44, 0.0_f64, -56.0_f64 / 81.0_f64 * t12161 * t1690 * t818 + 64.0_f64 / 27.0_f64 * t4070 * t19461 + 8.0_f64 / 27.0_f64 * t5481 * t824 - 16.0_f64 / 9.0_f64 * t817 * t35 * t595 - 8.0_f64 / 9.0_f64 * t1281 * t821 + 8.0_f64 / 3.0_f64 * t1281 * t2868 + 8.0_f64 / 27.0_f64 * t2898 * t1694 * t818 - 4.0_f64 / 9.0_f64 * t817 * t5455 * t234 - 2.0_f64 / 9.0_f64 * t5486 * t824 + t15072);
    t19482
}
