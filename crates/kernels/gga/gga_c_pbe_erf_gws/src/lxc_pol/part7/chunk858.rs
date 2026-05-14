//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 858/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk858<F: Float>(t2735: F, t611: F, t185: F, t4913: F, t5456: F, t1620: F, t1621: F, t1733: F, t5454: F, t155: F, t213: F, t1623: F, t2591: F, t644: F, t639: F, t1892: F, t5463: F) -> (F, F, F, F, F, F) {
    let t17499 = t2735 * t611;
    let t17500 = t185 * t17499;
    let t17501 = 64.0 / 405.0 * t17500;
    let t17503 = 32.0 / 5.0 * t4913 * t5456;
    let t17507 = 16.0 / 5.0 * t1620 * t1621 * t5454 * t1733;
    let t17508 = t155 * t213;
    let t17510 = t1620 * t17508 * t1623;
    let t17511 = 32.0 / 45.0 * t17510;
    let t17512 = t2591 * t644;
    let t17513 = t639 * t17512;
    let t17514 = 128.0 / 1215.0 * t17513;
    let t17516 = t639 * t5463 * t1892;
    (t17501, t17503, t17507, t17511, t17514, t17516)
}
