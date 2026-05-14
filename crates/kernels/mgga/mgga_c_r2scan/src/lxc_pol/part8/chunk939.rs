//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 939/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk939<F: Float>(t159: F, t8915: F, t617: F, t5331: F, t5335: F, t5338: F, t5340: F, t5344: F, t5346: F, t5350: F, t5354: F, t5355: F, t7708: F, t216: F, t5360: F, t5366: F, t5373: F, t5378: F, t7720: F, t7721: F, t7724: F, t7725: F, t7727: F, t7730: F, t7737: F, t8590: F) -> (F, F, F, F) {
    let t8916 = t159 * t8915;
    let t8917 = t8916 * t617;
    let t8925 = 0.84681398666666666666e-3 * t8917 + 16.0 * t7708 - t5331 + t5335 - 0.23392894490538584828e1 * t5338 + 0.34631718211362927518e2 * t5340 + 0.35089341735807877242e1 * t5344 - 0.10389515463408878255e3 * t5346 - t5350 - t5354 - 0.11696447245269292414e1 * t5355;
    let t8934 = -t5360 + t7720 - 0.21973736767207854065e-2 * t8590 * t216 + 0.20508037716432813315e4 * t7721 - t7724 - 0.46785788981077169656e1 * t7725 - 0.2602459512072417562e0 * t7727 + t7730 + t5366 + 0.1714584e0 * t5373 + 0.80040858019733333332e-2 * t5378 + 0.1301229756036208781e0 * t7737;
    (t8916, t8917, t8925, t8934)
}
