//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1192/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1192<F: Float>(t19609: F, t5047: F, t14832: F, t19593: F, t5077: F, t3337: F, t14838: F, t19588: F, t5076: F, t19614: F, t3438: F, t5175: F) -> (F, F, F, F) {
    let t19931 = t5047 * t19609;
    let t19932 = t14832 * t19931;
    let t19934 = t5077 * t19593;
    let t19935 = t3337 * t19934;
    let t19937 = t14838 * t19588;
    let t19938 = t5076 * t19937;
    let t19940 = t3438 * t19614;
    let t19941 = t5175 * t19940;
    (t19932, t19935, t19938, t19941)
}
