//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 951/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk951<F: Float>(t3888: F, t5384: F, t5385: F, t315: F, t442: F, t5386: F, t5340: F, t857: F, t181: F, t851: F, t4131: F, t872: F) -> (F, F, F, F, F) {
    let t15129 = t5384 * t5385 * t3888;
    let t15132 = t315 * t442 * t5386;
    let t15135 = t857 * t5340;
    let t15138 = t851 * t181 * t5386;
    let t15149 = t4131 * t872;
    (t15129, t15132, t15135, t15138, t15149)
}
