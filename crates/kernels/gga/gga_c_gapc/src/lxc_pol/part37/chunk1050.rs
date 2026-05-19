//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1050/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1050<F: Float>(t11567: F, t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11599: F, t11602: F, t11605: F, t1096: F, t11043: F, t3828: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12136 = F::cast_from(0.21720231316129303386e-4_f64) * t11567;
    let t12137 = F::cast_from(0.2318836277704281739e-4_f64) * t11570;
    let t12138 = F::cast_from(0.21720231316129303386e-4_f64) * t11572;
    let t12139 = F::cast_from(0.34752370105806885418e-3_f64) * t11574;
    let t12140 = F::cast_from(0.28960308421505737848e-5_f64) * t11581;
    let t12141 = F::cast_from(0.1349435763888888889e-4_f64) * t11584;
    let t12144 = F::cast_from(0.67530371184977617164e-6_f64) * t11599;
    let t12145 = F::cast_from(0.13506074236995523433e-5_f64) * t11602;
    let t12146 = F::cast_from(0.21103240995305505364e-7_f64) * t11605;
    let t12152 = t11043 * t1096;
    let t12153 = t3828 * t883;
    (t12136, t12137, t12138, t12139, t12140, t12141, t12144, t12145, t12146, t12152, t12153)
}
