//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1102/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1102<F: Float>(t10872: F, t11686: F, t10891: F, t11748: F, t261: F, t3304: F, t7233: F, t38182: F, t927: F, t2626: F, t503: F, t5119: F) -> (F, F, F, F, F) {
    let t39627 = t10872 * t11686;
    let t39629 = t11748 * t10891;
    let t39635 = t3304 * t261 * t7233;
    let t39637 = t38182 * t927;
    let t39640 = t503 * t5119 * t2626;
    (t39627, t39629, t39635, t39637, t39640)
}
