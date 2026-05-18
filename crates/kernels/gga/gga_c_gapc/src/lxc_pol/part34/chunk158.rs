//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 158/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk158<F: Float>(t159: F, t405: F, t104: F, t73: F, t14: F, t1: F, t108: F, t3: F, t78: F, t106: F, t70: F, t402: F) -> (F, F, F, F, F, F, F, F) {
    let t540 = t405 * t159;
    let t543 = t73 * t104;
    let t544 = t543 * t14;
    let t545 = t108 * t1;
    let t546 = t3 * t78;
    let t547 = t545 * t546;
    let t550 = t106 * t70;
    let t551 = t550 * t402;
    (t540, t543, t544, t545, t546, t547, t550, t551)
}
