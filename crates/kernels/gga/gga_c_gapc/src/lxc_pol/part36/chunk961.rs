//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 961/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk961<F: Float>(t11772: F, t29692: F, t11795: F, t9387: F, t11508: F, t3402: F, t7944: F, t11513: F, t7259: F, t11822: F, t7511: F, t11825: F, t3708: F, t9906: F, t3330: F, t10058: F, t11784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33565 = t11772 * t29692;
    let t33567 = t9387 * t11795;
    let t33570 = t3402 * t11508 * t7944;
    let t33576 = t7259 * t11513 * t7944;
    let t33578 = t11822 * t7511;
    let t33580 = t11825 * t7511;
    let t33582 = t9906 * t3708;
    let t33583 = t33582 * t3330;
    let t33585 = t11784 * t10058;
    (t33565, t33567, t33570, t33576, t33578, t33580, t33582, t33583, t33585)
}
