//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1238/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1238<F: Float>(t1014: F, t26843: F, t3245: F, t7727: F, t7735: F, t1094: F, t3169: F, t26773: F, t26778: F, t26797: F, t26848: F, t27084: F, t7784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92991 = t1014 * t26843;
    let t92993 = t3245 * t7727;
    let t92997 = t3245 * t7735;
    let t92999 = t3169 * t1094;
    let t93006 = t1014 * t26773;
    let t93008 = t1014 * t26778;
    let t93010 = t1014 * t26797;
    let t93012 = t1014 * t26848;
    let t93014 = t27084 * t7784;
    (t92991, t92993, t92997, t92999, t93006, t93008, t93010, t93012, t93014)
}
