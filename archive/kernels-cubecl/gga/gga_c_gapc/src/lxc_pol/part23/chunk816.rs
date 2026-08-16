//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 816/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk816<F: Float>(t3012: F, t7557: F, t2578: F, t1044: F, t1055: F, t311: F, t1074: F, t3271: F, t869: F, t1018: F, t876: F, t3272: F) -> (F, F, F, F, F, F) {
    let t9525 = t3012 * t7557;
    let t9526 = t2578 * t9525;
    let t9528 = t1055 * t1044;
    let t9529 = t311 * t9528;
    let t9530 = t9529 * t1074;
    let t9532 = t869 * t3271;
    let t9533 = t9532 * t1074;
    let t9535 = t1018 * t876;
    let t9536 = t3272 * t9535;
    (t9526, t9529, t9530, t9532, t9533, t9536)
}
