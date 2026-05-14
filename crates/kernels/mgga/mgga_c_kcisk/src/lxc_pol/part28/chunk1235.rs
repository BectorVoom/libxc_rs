//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1235/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1235<F: Float>(t6675: F, t7715: F, t9679: F, t5054: F, t8845: F, t9650: F, t7261: F) -> (F, F, F, F, F) {
    let t35131 = t6675 * t7715;
    let t35132 = t9679 * t35131;
    let t35133 = t5054 * t35132;
    let t35135 = t9650 * t8845;
    let t35136 = t7261 * t35135;
    (t35131, t35132, t35133, t35135, t35136)
}
