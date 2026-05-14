//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1109/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1109<F: Float>(t5073: F, t92515: F, t14714: F, t28029: F, t3226: F, t5177: F, t92514: F, t26891: F, t5062: F, t14735: F, t26930: F, t5099: F, t15086: F, t28024: F, t26938: F, t5078: F) -> (F, F, F, F, F, F, F, F) {
    let t95296 = t92515 * t5073;
    let t95298 = t28029 * t14714;
    let t95301 = t3226 * t92514 * t5177;
    let t95303 = t26891 * t5062;
    let t95305 = t26930 * t14735;
    let t95307 = t92515 * t5099;
    let t95309 = t28024 * t15086;
    let t95311 = t26938 * t5078;
    (t95296, t95298, t95301, t95303, t95305, t95307, t95309, t95311)
}
