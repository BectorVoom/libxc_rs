//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 376/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk376<F: Float>(t119: F, t155: F, t481: F, t1513: F, t1243: F, t486: F, t102: F, t128: F, t1504: F, t48: F, t1403: F, t1407: F, t476: F) -> (F, F, F, F, F, F, F) {
    let t1515 = t119 * t155 * t481;
    let t1516 = t1513 * t1515;
    let t1517 = F::new(0.97434166666666666666e0) * t1516;
    let t1519 = F::new(0.64956111111111111111e0) * t486 * t1243;
    let t1522 = F::new(0.584605e1) * t102 * t128 * t1504;
    let t1523 = F::new(1.0) / t48;
    let t1524 = t1523 * t1403;
    let t1526 = t476 * t1407;
    (t1515, t1517, t1519, t1522, t1523, t1524, t1526)
}
