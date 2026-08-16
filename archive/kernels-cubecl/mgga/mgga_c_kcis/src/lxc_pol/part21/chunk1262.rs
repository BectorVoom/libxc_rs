//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1262/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1262<F: Float>(t26941: F, t28045: F, t1796: F, t982: F, t7755: F, t26924: F, t5078: F, t26929: F, t5025: F, t3439: F, t14788: F, t7754: F) -> (F, F, F, F, F) {
    let t95374 = t28045 * t26941;
    let t95376 = t1796 * t982;
    let t95377 = t95376 * t7755;
    let t95379 = t26924 * t5078;
    let t95381 = t5025 * t26929;
    let t95382 = t95381 * t3439;
    let t95384 = t7754 * t14788;
    (t95374, t95377, t95379, t95382, t95384)
}
