//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1015/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1015<F: Float>(t6161: F, t816: F, t2182: F, t875: F, t3206: F, t858: F, t6672: F, t2112: F, t2138: F, t2306: F, t3074: F, t339: F, t360: F, t2195: F, t810: F, t2407: F) -> (F, F, F, F) {
    let t20441 = t816 * t6161;
    let t20449 = t875 * t2182;
    let t20451 = t3206 * t858 * t20449;
    let t20453 = 3.0 / 4.0 * t6672 * t20451;
    let t20459 = t3074 * t2306 * t2112 * t339 * t360 * t2138 / 8.0;
    let t20464 = t2195 * t810;
    let t20466 = t2407 * t858 * t20464;
    (t20441, t20453, t20459, t20466)
}
