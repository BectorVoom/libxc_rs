//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 697/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk697<F: Float>(t126: F, t1762: F, t611: F, t2975: F, t1932: F, t2979: F, t3085: F, t1: F, t5011: F, t5541: F, t102: F, t6: F, t134: F) -> (F, F, F, F, F, F, F) {
    let t8666 = t126 * t1762;
    let t8667 = t611 * t8666;
    let t8668 = t8667 * t2975;
    let t8670 = t1932 * t2979;
    let t8671 = t8670 * t3085;
    let t8673 = t5011 * t1;
    let t8674 = t5541 * t8673;
    let t8675 = t102 * t6;
    let t8676 = t8675 * t134;
    (t8666, t8668, t8671, t8673, t8674, t8675, t8676)
}
