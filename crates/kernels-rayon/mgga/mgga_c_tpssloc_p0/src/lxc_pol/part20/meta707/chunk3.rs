//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2701/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2701(t16468: f64, t225: f64, t16458: f64, t12023: f64, t12027: f64, t12033: f64, t12237: f64, t12444: f64, t1386: f64, t16022: f64, t16437: f64, t16453: f64, t16460: f64, t1834: f64, t1843: f64, t3752: f64, t3758: f64, t3882: f64, t3889: f64, t39910: f64, t5318: f64, t5321: f64, t5326: f64, t54738: f64, t562: f64, t568: f64) -> f64 {
    let t55134 = t16468 * t225;
    let t55150 = t16458 * t225;
    let t55155 = t12237 * t1834 * t568 + 3.0_f64 * t3752 * t5318 * t568 + t54738 * t562 * t568 - 6.0_f64 * t12023 * t5321 + 6.0_f64 * t12027 * t5321 + 6.0_f64 * t12033 * t5326 + 12.0_f64 * t12444 * t5326 - 3.0_f64 * t1386 * t55134 - 3.0_f64 * t1386 * t55150 + 6.0_f64 * t16022 * t3889 - 3.0_f64 * t16437 * t3882 + 12.0_f64 * t16453 * t3758 + 6.0_f64 * t16460 * t3889 - t1843 * t39910;
    t55155
}
