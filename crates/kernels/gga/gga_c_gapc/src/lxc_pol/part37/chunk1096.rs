//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1096/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1096<F: Float>(t144: F, t4043: F, t674: F, t3707: F, t5972: F, t647: F, t137: F, t5: F, t4: F, t5971: F, t11589: F, t102: F, t198: F) -> (F, F, F, F, F, F, F) {
    let t20372 = t4043 * t144 * t674;
    let t20461 = t3707 * t144;
    let t20487 = t647 * t5972;
    let t20499 = t5 * t137;
    let t20500 = t20499 * t4;
    let t20501 = t5971 * t20500;
    let t20563 = t11589 * t137;
    let t20569 = t102 * t198 * t674;
    (t20372, t20461, t20487, t20500, t20501, t20563, t20569)
}
