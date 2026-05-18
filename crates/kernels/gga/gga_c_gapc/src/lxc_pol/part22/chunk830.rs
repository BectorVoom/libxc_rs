//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 830/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk830<F: Float>(t2979: F, t8117: F, t3388: F, t2520: F, t3392: F, t128: F, t147: F, t19: F, t2535: F, t919: F, t1084: F, t3717: F) -> (F, F, F, F, F) {
    let t9490 = t8117 * t2979;
    let t9491 = t9490 * t3388;
    let t9493 = t2520 * t2979;
    let t9494 = t9493 * t3392;
    let t9497 = t128 * t19 * t147;
    let t9499 = t2535 * t919 * t9497;
    let t9501 = t1084 * t3717;
    (t9491, t9494, t9497, t9499, t9501)
}
