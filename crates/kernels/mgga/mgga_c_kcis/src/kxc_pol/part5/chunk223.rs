//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 223/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk223<F: Float>(t113: F, t122: F, t60: F, t684: F, t718: F, t728: F, t745: F, t97: F) -> (F,) {
    let t747 = -0.11713266981940447749e-2 * t113 * t97 - 0.23426533963880895498e-2 * t718 * t728 - t684 * t122 - t60 * t745;
    (t747,)
}
