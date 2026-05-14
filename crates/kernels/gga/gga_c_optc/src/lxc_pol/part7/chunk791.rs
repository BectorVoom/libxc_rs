//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 791/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk791<F: Float>(t7253: F, t7256: F, t6534: F, t322: F, t2623: F, t2630: F, t2635: F, t289: F, t314: F, t7880: F, t7883: F, t7886: F, t7889: F, t7891: F, t7897: F, t7899: F, t7902: F, t7904: F, t7907: F, t7915: F, t7918: F, t7921: F, t862: F, t899: F) -> (F, F, F, F) {
    let t7924 = t7253 * t7256;
    let t7925 = t7924 * t6534;
    let t7926 = t322 * t7925;
    let t7929 = -0.12073835402484385909e-2 * t7880 - 0.72443012414906315455e-2 * t7883 + 0.18352229811776266582e0 * t7886 * t899 - 0.3863627328795003491e-1 * t7889 + t862 * t7891 / 72.0 + t7897 - 0.67291509309846310801e0 * t7899 * t314 + 0.18352229811776266582e0 * t7902 + 0.96590683219875087275e-2 * t7904 - 77.0 / 162.0 * t7907 * t289 - t2623 * t2630 / 36.0 - t2623 * t2635 / 27.0 + t7915 / 288.0 + t7918 / 216.0 + t862 * t7921 / 288.0 + 7.0 / 648.0 * t862 * t7926;
    (t7924, t7925, t7926, t7929)
}
