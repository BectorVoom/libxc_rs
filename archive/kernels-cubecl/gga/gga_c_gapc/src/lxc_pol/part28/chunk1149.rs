//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1149/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1149<F: Float>(t1084: F, t1971: F, t9841: F, t9846: F, t11910: F, t29350: F, t125: F, t8448: F, t9059: F, t10293: F, t28524: F, t28526: F) -> (F, F, F, F) {
    let t33394 = t1084 * t1971 * t9841 * t9846;
    let t33396 = t11910 * t29350;
    let t33399 = t9059 * t8448 * t125;
    let t33402 = t28524 * t33399 * t10293 * t28526;
    (t33394, t33396, t33399, t33402)
}
