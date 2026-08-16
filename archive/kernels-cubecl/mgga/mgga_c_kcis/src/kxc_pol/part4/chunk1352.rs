//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1352/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1352<F: Float>(t16658: F, t5909: F, t5908: F, t2047: F, t4245: F, t12568: F, t5932: F, t16653: F, t4261: F, t4260: F, t492: F, t6015: F) -> (F, F, F, F, F) {
    let t17402 = t5909 * t16658;
    let t17403 = t5908 * t17402;
    let t17405 = t4245 * t2047;
    let t17407 = t12568 * t5932;
    let t17409 = t4261 * t16653;
    let t17410 = t4260 * t17409;
    let t17412 = t6015 * t492;
    (t17403, t17405, t17407, t17410, t17412)
}
