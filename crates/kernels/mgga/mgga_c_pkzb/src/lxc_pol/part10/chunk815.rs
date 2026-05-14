//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 815/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk815<F: Float>(t114: F, t4920: F, t1507: F, t4913: F, t47: F, t58: F, t69: F, t82: F, t572: F, t66: F) -> (F, F, F, F, F) {
    let t4921 = t114 * t4920;
    let t4922 = t4913 * t1507;
    let t4928 = 1.0 / t58 / t69 * t47 / 4.0;
    let t4929 = t4928 * t82;
    let t4932 = 1.0 / t66 / t572;
    (t4921, t4922, t4928, t4929, t4932)
}
