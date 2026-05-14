//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1146/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1146<F: Float>(t1307: F, t17730: F, t6159: F, t18175: F, t27583: F, t28766: F, t16685: F, t27607: F, t28765: F, t28772: F, t28807: F, t6151: F, t94928: F, t98171: F, t98179: F, t98188: F, t99098: F, t99100: F, t99108: F) -> (F, F) {
    let t99110 = t6159 * t17730 * t1307;
    let t99117 = 0.10297067901234567901e-3 * t27583 * t18175 * t28766;
    let t99118 = -0.38691203703703703703e-3 * t98171 + 0.38691203703703703703e-3 * t98179 + 0.69505208333333333334e-3 * t27607 * t28772 - t99098 - t99100 + 0.19345601851851851852e-2 * t98188 - 0.15445601851851851852e-3 * t27583 * t6151 * t28765 * t16685 + t99108 + 0.23168402777777777778e-3 * t27583 * t99110 + 0.23168402777777777778e-3 * t94928 * t28807 - t99117;
    (t99110, t99118)
}
