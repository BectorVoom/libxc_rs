//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1237/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1237<F: Float>(t6106: F, t997: F, t1886: F, t3670: F, t5681: F, t1008: F, t3372: F, t5727: F, t1165: F, t12801: F, t16559: F, t5852: F) -> (F, F, F, F, F, F) {
    let t22621 = t997 * t6106;
    let t22623 = t3670 * t1886;
    let t22625 = t997 * t5681;
    let t22627 = t1008 * t5681;
    let t22633 = t3372 * t5727;
    let t22642 = t12801 * t1165 * t5852 * t16559;
    (t22621, t22623, t22625, t22627, t22633, t22642)
}
