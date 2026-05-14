//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 851/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk851<F: Float>(t11185: F, t3651: F, t632: F, t996: F, t3634: F, t458: F, t568: F, t997: F, t437: F, t516: F, t8356: F, t125: F, t515: F, t619: F, t2903: F, t101: F, t3643: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11186 = t3651 * t11185;
    let t11188 = t996 * t632;
    let t11189 = t3634 * t458;
    let t11190 = t11188 * t11189;
    let t11192 = t3634 * t568;
    let t11193 = t997 * t11192;
    let t11195 = t516 * t437;
    let t11196 = t8356 * t11195;
    let t11198 = t515 * t125;
    let t11199 = t11198 * t619;
    let t11200 = t2903 * t11199;
    let t11202 = t3643 * t101;
    (t11186, t11188, t11189, t11190, t11192, t11193, t11195, t11196, t11198, t11199, t11200, t11202)
}
