//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1097/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1097<F: Float>(t38182: F, t927: F, t2626: F, t503: F, t5119: F, t2842: F, t37699: F, t10698: F, t2593: F, t38152: F, t7418: F, t38149: F, t39469: F) -> (F, F, F, F, F, F) {
    let t39637 = t38182 * t927;
    let t39640 = t503 * t5119 * t2626;
    let t39642 = t37699 * t2842;
    let t39672 = t10698 * t2593;
    let t39673 = F::cast_from(0.25610080155860322884e0_f64) * t39672;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    (t39637, t39640, t39642, t39673, t39721, t39723)
}
