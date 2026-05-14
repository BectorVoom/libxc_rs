//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 791/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk791<F: Float>(t1062: F, t2393: F, t721: F, t2304: F, t3743: F, t1106: F, t8092: F, t1067: F, t2394: F, t2380: F, t1026: F, t115: F, t2341: F, t4088: F, t4426: F, t4494: F, t4497: F, t4499: F, t4504: F, t4512: F, t4519: F, t713: F, t8977: F, t9159: F) -> (F,) {
    let t9512 = t2393 * t1062;
    let t9513 = t9512 * t721;
    let t9515 = t2304 * t3743;
    let t9522 = t1106 * t8092;
    let t9526 = t2394 * t1067;
    let t9537 = t2380 * t1067;
    let t9539 = t9513 / 6.0 - t9515 * t4494 / 18.0 - t9515 * t4426 / 18.0 + t9515 * t4088 / 18.0 + t9522 / 6.0 - t8977 * t713 / 6.0 + t9526 / 9.0 - t1026 * t2341 / 6.0 + t115 * t9159 / 6.0 - t4497 / 6.0 - t4499 / 6.0 + t4504 / 6.0 - t4512 / 6.0 + t4519 / 6.0 + t9537 / 9.0;
    (t9539,)
}
