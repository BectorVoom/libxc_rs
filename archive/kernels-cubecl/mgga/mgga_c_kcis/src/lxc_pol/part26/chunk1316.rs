//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1316/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1316<F: Float>(t22213: F, t303: F, t7931: F, t102278: F, t28747: F, t95024: F, t1610: F, t6281: F, t1615: F, t6159: F, t95103: F, t21854: F, t4160: F, t98266: F) -> (F, F, F, F, F, F) {
    let t102563 = t303 * t7931 * t22213;
    let t102568 = t95024 * t102278 * t28747;
    let t102575 = t6281 * t1610;
    let t102580 = t6281 * t1615;
    let t102582 = t6159 * t95103 * t102580;
    let t102586 = t4160 * t98266 * t21854;
    (t102563, t102568, t102575, t102580, t102582, t102586)
}
