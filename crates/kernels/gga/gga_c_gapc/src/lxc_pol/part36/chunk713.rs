//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 713/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk713<F: Float>(t1734: F, t9244: F, t1030: F, t2997: F, t674: F, t3142: F, t5319: F, t3137: F, t1461: F, t2993: F, t1038: F, t5972: F, t3712: F) -> (F, F, F, F, F, F, F, F) {
    let t9245 = t1734 * t9244;
    let t9246 = t1030 * t9245;
    let t9247 = t2997 * t674;
    let t9248 = t5319 * t3142;
    let t9249 = t9247 * t9248;
    let t9250 = t9246 * t9249;
    let t9252 = t3137 * M_PI;
    let t9253 = t1461 * t9252;
    let t9254 = t2993 * t9253;
    let t9255 = t1038 * t5972;
    let t9256 = t3712 * t9255;
    (t9245, t9249, t9250, t9252, t9253, t9254, t9255, t9256)
}
