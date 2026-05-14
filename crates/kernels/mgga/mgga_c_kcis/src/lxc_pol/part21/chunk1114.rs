//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1114/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1114<F: Float>(t1021: F, t14775: F, t26941: F, t28045: F, t1796: F, t982: F, t7755: F, t26924: F, t5078: F, t26929: F, t5025: F, t3439: F, t14788: F, t7754: F, t95361: F, t95364: F, t95366: F, t95368: F, t95370: F) -> (F, F, F, F, F, F, F) {
    let t95372 = t1021 * t14775;
    let t95374 = t28045 * t26941;
    let t95376 = t1796 * t982;
    let t95377 = t95376 * t7755;
    let t95379 = t26924 * t5078;
    let t95381 = t5025 * t26929;
    let t95382 = t95381 * t3439;
    let t95384 = t7754 * t14788;
    let t95386 = 19.0 / 72.0 * t95361 - t95364 / 16.0 - t95366 / 288.0 + t95368 / 9.0 + t95370 / 9.0 - 19.0 / 54.0 * t95372 + t95374 / 24.0 - 2.0 / 9.0 * t95377 + 2.0 / 27.0 * t95379 + t95382 / 48.0 - t95384 / 72.0;
    (t95372, t95374, t95377, t95379, t95382, t95384, t95386)
}
