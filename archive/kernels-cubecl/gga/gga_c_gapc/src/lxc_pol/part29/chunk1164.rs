//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1164/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1164<F: Float>(t1036: F, t11316: F, t13483: F, t11503: F, t9041: F, t11387: F, t3060: F, t3123: F, t1423: F, t3115: F, t3116: F, t11388: F, t9050: F) -> (F, F, F, F, F, F) {
    let t34378 = t11316 * t1036 * t13483;
    let t34380 = t9041 * t11503;
    let t34382 = t3060 * t11387;
    let t34383 = t34382 * t3123;
    let t34386 = t3115 * t1423 * t3116;
    let t34388 = t11388 * t9050;
    (t34378, t34380, t34382, t34383, t34386, t34388)
}
