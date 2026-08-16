//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 902/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk902<F: Float>(t125: F, t515: F, t619: F, t2903: F, t101: F, t3643: F, t128: F, t1458: F, t8297: F, t19: F, t8286: F, t147: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11198 = t515 * t125;
    let t11199 = t11198 * t619;
    let t11200 = t2903 * t11199;
    let t11202 = t3643 * t101;
    let t11203 = t1458 * t128;
    let t11204 = t11202 * t11203;
    let t11205 = t11204 * t8297;
    let t11207 = t1458 * t19;
    let t11208 = t8286 * t11207;
    let t11209 = t147 * t125;
    (t11198, t11199, t11200, t11202, t11203, t11204, t11205, t11207, t11208, t11209)
}
