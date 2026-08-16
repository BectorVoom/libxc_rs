//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1025/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1025<F: Float>(t15562: F, t5329: F, t3530: F, t5336: F, t1262: F, t25: F, t287: F) -> (F, F, F) {
    let t15563 = t5329 * t15562;
    let t15568 = t3530 * t5336;
    let t15569 = t15568 * t1262;
    let t15570 = t5329 * t15569;
    let t15573 = t25 * t287;
    (t15563, t15570, t15573)
}
