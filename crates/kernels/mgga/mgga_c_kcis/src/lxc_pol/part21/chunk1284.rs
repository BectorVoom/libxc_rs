//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1284/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1284<F: Float>(t283: F, t9588: F, t1092: F, t1800: F, t3228: F, t27764: F, t3226: F, t982: F, t5025: F, t26762: F, t1009: F, t4972: F) -> (F, F, F, F, F) {
    let t95655 = t9588 * t283;
    let t95658 = t1092 * t95655 * t1800 * t3228;
    let t95662 = t1092 * t3226 * t982 * t27764;
    let t95664 = t5025 * t283;
    let t95666 = t1092 * t95664 * t26762;
    let t95670 = t1009 * t4972;
    (t95658, t95662, t95664, t95666, t95670)
}
