//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 678/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk678<F: Float>(t2661: F, t7371: F, t2263: F, t864: F, t2548: F, t7298: F, t312: F, t9: F, t116: F, t7328: F, t286: F, t2666: F, t311: F) -> (F, F, F, F, F, F) {
    let t7386 = t2661 * t7371;
    let t7397 = t864 * t2263;
    let t7405 = t2548 * t7298;
    let t7433 = t9 * t312;
    let t7445 = t116 * t7328;
    let t7447 = 5.0 / 1296.0 * t286 * t7445;
    let t7448 = t2666 * t311;
    (t7386, t7397, t7405, t7433, t7447, t7448)
}
