//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 898/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk898<F: Float>(t1423: F, t1464: F, t3651: F, t632: F, t996: F, t3634: F, t458: F, t568: F, t997: F, t437: F, t516: F, t8356: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11185 = t1423 * t1464;
    let t11186 = t3651 * t11185;
    let t11188 = t996 * t632;
    let t11189 = t3634 * t458;
    let t11190 = t11188 * t11189;
    let t11192 = t3634 * t568;
    let t11193 = t997 * t11192;
    let t11195 = t516 * t437;
    let t11196 = t8356 * t11195;
    (t11185, t11186, t11188, t11189, t11190, t11192, t11193, t11195, t11196)
}
