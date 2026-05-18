//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1020/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1020<F: Float>(t1153: F, t2417: F, t6851: F, t869: F, t291: F, t3707: F, t1180: F, t7451: F, t2579: F, t891: F, t2232: F, t2546: F) -> (F, F, F, F, F, F) {
    let t16404 = t2417 * t1153;
    let t16408 = t869 * t6851;
    let t16471 = t3707 * t291;
    let t16676 = t7451 * t1180;
    let t16677 = t2579 * t891;
    let t16720 = t2546 * t2232;
    (t16404, t16408, t16471, t16676, t16677, t16720)
}
