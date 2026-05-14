//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1115/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1115<F: Float>(t8069: F, t92486: F, t26929: F, t9588: F, t14850: F, t8072: F, t92532: F, t26891: F, t5091: F, t14812: F, t28029: F, t1176: F, t5164: F, t26933: F, t3448: F, t4999: F) -> (F, F, F, F, F, F, F, F) {
    let t95389 = t92486 * t8069;
    let t95391 = t9588 * t26929;
    let t95392 = t95391 * t14850;
    let t95394 = t92532 * t8072;
    let t95396 = t26891 * t5091;
    let t95398 = t28029 * t14812;
    let t95400 = t5164 * t1176;
    let t95402 = t26933 * t5091;
    let t95404 = t4999 * t3448;
    (t95389, t95392, t95394, t95396, t95398, t95400, t95402, t95404)
}
