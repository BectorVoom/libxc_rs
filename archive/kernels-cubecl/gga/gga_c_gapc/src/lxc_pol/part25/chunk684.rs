//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 684/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk684<F: Float>(t1709: F, t618: F, t567: F, t635: F, t144: F, t1908: F) -> (F, F, F) {
    let t5858 = t618 * t1709;
    let t5918 = t635 * t567;
    let t5963 = t144 * t1908;
    (t5858, t5918, t5963)
}
