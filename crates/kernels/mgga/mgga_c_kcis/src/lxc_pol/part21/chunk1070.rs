//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1070/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1070<F: Float>(t36908: F, t695: F, t92016: F, t2725: F, t7639: F, t9194: F, t26477: F, t26480: F, t26474: F, t26501: F, t7642: F, t209: F, t2155: F, t8779: F, t8780: F, t888: F) -> (F, F, F, F, F, F) {
    let t92022 = t36908 * t695 * t92016;
    let t92025 = t2725 * t9194 * t7639;
    let t92027 = t26480 * t26477;
    let t92029 = t26474 * t92016;
    let t92031 = t7642 * t26501;
    let t92036 = t2155 * t209 * t8779 * t888 * t8780;
    (t92022, t92025, t92027, t92029, t92031, t92036)
}
