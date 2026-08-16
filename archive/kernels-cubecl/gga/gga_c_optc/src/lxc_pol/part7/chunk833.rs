//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 833/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk833<F: Float>(t322: F, t7925: F, t2623: F, t2630: F, t2635: F, t289: F, t314: F, t7880: F, t7883: F, t7886: F, t7889: F, t7891: F, t7897: F, t7899: F, t7902: F, t7904: F, t7907: F, t7915: F, t7918: F, t7921: F, t862: F, t899: F) -> (F, F) {
    let t7926 = t322 * t7925;
    let t7929 = -F::cast_from(0.12073835402484385909e-2_f64) * t7880 - F::cast_from(0.72443012414906315455e-2_f64) * t7883 + F::cast_from(0.18352229811776266582e0_f64) * t7886 * t899 - F::cast_from(0.3863627328795003491e-1_f64) * t7889 + t862 * t7891 / F::cast_from(72.0_f64) + t7897 - F::cast_from(0.67291509309846310801e0_f64) * t7899 * t314 + F::cast_from(0.18352229811776266582e0_f64) * t7902 + F::cast_from(0.96590683219875087275e-2_f64) * t7904 - F::cast_from(77.0_f64) / F::cast_from(162.0_f64) * t7907 * t289 - t2623 * t2630 / F::cast_from(36.0_f64) - t2623 * t2635 / F::cast_from(27.0_f64) + t7915 / F::cast_from(288.0_f64) + t7918 / F::cast_from(216.0_f64) + t862 * t7921 / F::cast_from(288.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t862 * t7926;
    (t7926, t7929)
}
