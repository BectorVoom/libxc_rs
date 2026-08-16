//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2695/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2695(t19635: f64, t225: f64, t20048: f64, t1375: f64, t1386: f64, t16022: f64, t16030: f64, t16122: f64, t16436: f64, t16460: f64, t16471: f64, t16475: f64, t1834: f64, t1842: f64, t19648: f64, t20026: f64, t3758: f64, t3879: f64, t3882: f64, t3887: f64, t3888: f64, t3911: f64, t40591: f64, t5210: f64, t5215: f64, t5318: f64, t5321: f64, t5326: f64, t5354: f64, t568: f64, t6361: f64, t6439: f64, t6460: f64) -> f64 {
    let t56607 = t19635 * t225;
    let t56640 = t20048 * t225;
    let t56649 = -4.0_f64 * t56607 * t1386 + 8.0_f64 * t16030 * t5326 + 4.0_f64 * t5321 * t16471 + 4.0_f64 * t1375 * t3887 * t1842 * t16436 - 12.0_f64 * t5215 * t16475 + 2.0_f64 * t16122 * t1834 * t568 + 4.0_f64 * t5210 * t5318 * t568 + 24.0_f64 * t1375 * t40591 * t6439 * t3888 - 4.0_f64 * t16460 * t5354 + 4.0_f64 * t3758 * t20026 + 8.0_f64 * t16022 * t5326 + 2.0_f64 * t1375 * t3887 * t6460 * t3911 - 2.0_f64 * t56640 * t1386 + t6361 * t3879 * t568 + 8.0_f64 * t16460 * t5326 + 8.0_f64 * t3882 * t19648;
    t56649
}
