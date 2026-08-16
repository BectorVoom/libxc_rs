//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2736/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736(t12021: f64, t12030: f64, t1375: f64, t1378: f64, t16030: f64, t16413: f64, t16437: f64, t16439: f64, t16453: f64, t16471: f64, t1807: f64, t1843: f64, t19648: f64, t20051: f64, t20060: f64, t3758: f64, t3882: f64, t3888: f64, t3889: f64, t3911: f64, t5215: f64, t5321: f64, t5326: f64, t5354: f64, t539: f64, t55134: f64, t568: f64, t57485: f64, t57526: f64, t57564: f64, t57597: f64, t57631: f64, t57667: f64, t57692: f64, t57725: f64, t57760: f64, t6439: f64, t6440: f64, t6460: f64) -> f64 {
    let t57795 = -4.0_f64 * t16439 * t5354 + t539 * t57485 * t568 + 4.0_f64 * t5215 * t16471 + 2.0_f64 * t20060 * t3889 - t1375 * t1378 * (t57526 + t57564 + t57597 + t57631 + t57667 + t57692 + t57725 + t57760) + 2.0_f64 * t12030 * t6440 - 2.0_f64 * t55134 * t1843 - 12.0_f64 * t3758 * t20051 - 6.0_f64 * t1375 * t12021 * t6460 * t3888 - 12.0_f64 * t3882 * t20051 + 8.0_f64 * t3758 * t19648 - 6.0_f64 * t1375 * t12021 * t6439 * t3911 - 4.0_f64 * t16030 * t5354 + 2.0_f64 * t1807 * t16413 * t568 + 8.0_f64 * t5321 * t16453 - 2.0_f64 * t5215 * t16437 + 8.0_f64 * t16439 * t5326;
    t57795
}
