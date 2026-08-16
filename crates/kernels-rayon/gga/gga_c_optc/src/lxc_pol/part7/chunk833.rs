//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 833/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk833(t322: f64, t7925: f64, t2623: f64, t2630: f64, t2635: f64, t289: f64, t314: f64, t7880: f64, t7883: f64, t7886: f64, t7889: f64, t7891: f64, t7897: f64, t7899: f64, t7902: f64, t7904: f64, t7907: f64, t7915: f64, t7918: f64, t7921: f64, t862: f64, t899: f64) -> (f64, f64) {
    let t7926 = t322 * t7925;
    let t7929 = -0.12073835402484385909e-2_f64 * t7880 - 0.72443012414906315455e-2_f64 * t7883 + 0.18352229811776266582e0_f64 * t7886 * t899 - 0.3863627328795003491e-1_f64 * t7889 + t862 * t7891 / 72.0_f64 + t7897 - 0.67291509309846310801e0_f64 * t7899 * t314 + 0.18352229811776266582e0_f64 * t7902 + 0.96590683219875087275e-2_f64 * t7904 - 77.0_f64 / 162.0_f64 * t7907 * t289 - t2623 * t2630 / 36.0_f64 - t2623 * t2635 / 27.0_f64 + t7915 / 288.0_f64 + t7918 / 216.0_f64 + t862 * t7921 / 288.0_f64 + 7.0_f64 / 648.0_f64 * t862 * t7926;
    (t7926, t7929)
}
