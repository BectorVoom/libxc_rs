//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 719/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk719<F: Float>(t132: F, t26078: F, t36: F, t4787: F, t638: F, t71: F, t2084: F, t27: F, t7282: F, t794: F, t2160: F, t7224: F, t2184: F, t465: F, t7472: F, t7478: F) -> (F, F, F, F, F, F) {
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36715 = t7282 * t27 * t2084 * t794;
    let t36718 = t638 * t2160 * t7224;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36735 = t36734 * t7478;
    (t36700, t36715, t36718, t36733, t36734, t36735)
}
