//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1043/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1043<F: Float>(t11923: F, t30158: F, t3402: F, t10036: F, t11872: F, t11960: F, t869: F, t9555: F, t11965: F, t9741: F, t311: F, t34159: F, t7089: F, t919: F, t2415: F, t3439: F, t9756: F) -> (F, F, F, F, F, F) {
    let t34222 = t3402 * t11923 * t30158;
    let t34224 = t11872 * t10036;
    let t34227 = t869 * t11960 * t9555;
    let t34230 = t869 * t11965 * t9741;
    let t34235 = t311 * t7089 * t34159 * t919;
    let t34238 = t9756 * t2415 * t3439;
    (t34222, t34224, t34227, t34230, t34235, t34238)
}
