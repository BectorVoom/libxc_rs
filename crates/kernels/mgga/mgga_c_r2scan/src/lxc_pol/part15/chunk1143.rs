//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1143/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1143<F: Float>(t11675: F, t19872: F, t11678: F, t6395: F, t10764: F, t26282: F, t10882: F, t11748: F, t38152: F, t7418: F, t38149: F, t39469: F) -> (F, F, F, F, F, F) {
    let t39713 = t19872 * t11675;
    let t39715 = t6395 * t11678;
    let t39717 = t26282 * t10764;
    let t39719 = t11748 * t10882;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    (t39713, t39715, t39717, t39719, t39721, t39723)
}
