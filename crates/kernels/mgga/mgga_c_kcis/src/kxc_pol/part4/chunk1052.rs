//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1052/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1052<F: Float>(t339: F, t9368: F, t13467: F, t13516: F, t5134: F, t1045: F, t934: F, t4606: F, t13618: F, t304: F, t238: F, t5158: F, t86: F, t13495: F, t5142: F, t1728: F) -> (F, F, F, F, F, F, F) {
    let t15022 = t9368 * t339;
    let t15023 = t15022 * t13467;
    let t15026 = t5134 * t13516;
    let t15036 = t934 * t1045;
    let t15037 = t4606 * t15036;
    let t15040 = t304 * t13618;
    let t15046 = 0.53062222222222222222e-1 * t86 * t238 * t5158;
    let t15047 = t5142 * t13495;
    let t15050 = t1728 * t15036;
    (t15023, t15026, t15037, t15040, t15046, t15047, t15050)
}
