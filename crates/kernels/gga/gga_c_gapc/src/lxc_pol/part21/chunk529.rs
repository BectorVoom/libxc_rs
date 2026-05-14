//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 529/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk529<F: Float>(t1066: F, t883: F, t1096: F, t972: F, t1055: F, t644: F, t311: F, t442: F, t906: F) -> (F, F, F, F, F) {
    let t3265 = t1066 * t883;
    let t3268 = t1096 * t972;
    let t3271 = t1055 * t644;
    let t3272 = t311 * t3271;
    let t3273 = t442 * t906;
    (t3265, t3268, t3271, t3272, t3273)
}
