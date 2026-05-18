//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 964/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk964<F: Float>(t3379: F, t4979: F, t2450: F, t3371: F, t4737: F, t14056: F, t4419: F, t3706: F, t513: F, t1165: F, t3290: F, t3391: F) -> (F, F, F, F, F, F) {
    let t15479 = t3379 * t4979;
    let t15482 = t2450 * t3371;
    let t15483 = t15482 * t4737;
    let t15486 = t14056 * t4419;
    let t15494 = t3706 * t513;
    let t15497 = t3391 * t1165 * t15494 * t3290;
    (t15479, t15482, t15483, t15486, t15494, t15497)
}
