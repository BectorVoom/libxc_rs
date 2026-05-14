//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 716/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk716<F: Float>(t2315: F, t6203: F, t2074: F, t6: F, t254: F, t906: F, t745: F, t810: F, t2255: F, t851: F, t2132: F, t2306: F, t2382: F, t2138: F, t343: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6204 = t6203 * t2315;
    let t6206 = t6 * t2074;
    let t6207 = t254 * t6206;
    let t6208 = t6207 * t906;
    let t6211 = t745 * t810;
    let t6213 = t2255 * t851 * t6211;
    let t6216 = t2306 * t2132;
    let t6217 = t2382 * t6216;
    let t6219 = t6217 * t2138 / 32.0;
    let t6220 = t343 * t2074;
    (t6204, t6206, t6207, t6208, t6211, t6213, t6217, t6219, t6220)
}
