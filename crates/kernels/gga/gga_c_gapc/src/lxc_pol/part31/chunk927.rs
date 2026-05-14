//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 927/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk927<F: Float>(t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11599: F, t11602: F, t11605: F, t1096: F, t11043: F, t3828: F, t883: F, t972: F, t1125: F, t9375: F, t3449: F, t3565: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12137 = 0.2318836277704281739e-4 * t11570;
    let t12138 = 0.21720231316129303386e-4 * t11572;
    let t12139 = 0.34752370105806885418e-3 * t11574;
    let t12140 = 0.28960308421505737848e-5 * t11581;
    let t12141 = 0.1349435763888888889e-4 * t11584;
    let t12144 = 0.67530371184977617164e-6 * t11599;
    let t12145 = 0.13506074236995523433e-5 * t11602;
    let t12146 = 0.21103240995305505364e-7 * t11605;
    let t12152 = t11043 * t1096;
    let t12153 = t3828 * t883;
    let t12154 = t12153 * t972;
    let t12155 = t9375 * t1125;
    let t12156 = t3565 * t3449;
    (t12137, t12138, t12139, t12140, t12141, t12144, t12145, t12146, t12152, t12153, t12154, t12155, t12156)
}
