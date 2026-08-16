//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 428/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk428<F: Float>(t1397: F, t1428: F, t2366: F, t486: F, t1423: F, t1: F, t594: F, t106: F, t544: F) -> (F, F, F, F, F, F) {
    let t4379 = t1397 * t1428;
    let t4385 = t486 * t2366;
    let t4386 = t1423 * t4385;
    let t4389 = t594 * t1;
    let t4390 = t4389 * t106;
    let t4391 = t544 * t4390;
    (t4379, t4385, t4386, t4389, t4390, t4391)
}
