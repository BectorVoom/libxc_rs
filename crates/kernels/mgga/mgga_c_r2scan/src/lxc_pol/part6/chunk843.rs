//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 843/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk843<F: Float>(t166: F, t6044: F, t759: F, t4845: F, t4873: F, t5024: F, t5026: F, t5028: F, t5030: F, t5033: F, t5035: F, t5039: F, t6021: F, t6026: F, t6030: F, t6033: F, t6036: F, t6039: F, t765: F) -> (F, F, F) {
    let t6045 = t166 * t6044;
    let t6047 = 0.285764e-1 * t759 * t6045;
    let t6048 = t4845 + 0.2025780996e0 * t765 * t6021 - t6026 - t5024 + 0.4051561992e0 * t6030 - 0.2025780996e0 * t6033 - t5026 + t5028 + t5030 - t4873 + 0.857292e-1 * t6036 + 0.857292e-1 * t6039 + t6047 - t5033 - t5035 - t5039;
    (t6045, t6047, t6048)
}
