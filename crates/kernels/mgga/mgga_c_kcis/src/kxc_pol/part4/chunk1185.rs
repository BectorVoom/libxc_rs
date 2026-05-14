//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1185/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1185<F: Float>(t17391: F, t4294: F, t2066: F, t4278: F, t2033: F, t4121: F, t4257: F, t12530: F, t5913: F, t16658: F, t5909: F, t5908: F, t2047: F, t4245: F, t12568: F, t5932: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t17392 = t17391 * t4294;
    let t17394 = t4278 * t2066;
    let t17396 = t2033 * t4121;
    let t17397 = t17396 * sigma2;
    let t17398 = t17397 * t4257;
    let t17400 = t12530 * t5913;
    let t17402 = t5909 * t16658;
    let t17403 = t5908 * t17402;
    let t17405 = t4245 * t2047;
    let t17407 = t12568 * t5932;
    (t17392, t17394, t17398, t17400, t17403, t17405, t17407)
}
