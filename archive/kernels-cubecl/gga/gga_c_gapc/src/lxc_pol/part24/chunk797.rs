//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 797/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk797<F: Float>(t3303: F, t9520: F, t3300: F, t7553: F, t3012: F, t7557: F, t2578: F, t1044: F, t1055: F, t311: F, t1074: F, t3271: F, t869: F) -> (F, F, F, F, F, F) {
    let t9521 = t3303 * t9520;
    let t9523 = t7553 * t3300;
    let t9525 = t3012 * t7557;
    let t9526 = t2578 * t9525;
    let t9528 = t1055 * t1044;
    let t9529 = t311 * t9528;
    let t9530 = t9529 * t1074;
    let t9532 = t869 * t3271;
    (t9521, t9523, t9526, t9529, t9530, t9532)
}
