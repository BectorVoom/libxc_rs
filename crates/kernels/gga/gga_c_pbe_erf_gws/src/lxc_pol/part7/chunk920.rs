//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 920/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk920<F: Float>(t40: F, t427: F, t4742: F, t1423: F, t1438: F, t1275: F, t1285: F, t4659: F, t4661: F, t4358: F, t461: F, t409: F, t4832: F, t18467: F, t18471: F, t18474: F, t18477: F, t18479: F, t18512: F, t18514: F, t18518: F) -> (F, F, F, F, F, F) {
    let t18520 = t40 * t427 * t4742;
    let t18521 = 4.0 * t18520;
    let t18522 = t1438 * t1423;
    let t18523 = 192.0 * t18522;
    let t18527 = 0.3103500882342370105e4 * t4659 * t1275 * t4661 * t1285;
    let t18528 = t4358 * t461;
    let t18529 = 96.0 * t18528;
    let t18530 = t409 * t4832;
    let t18531 = 16.0 * t18530;
    let t18532 = t18467 - t18471 - t18474 + t18477 + t18479 + t18512 - t18514 + t18518 + t18521 - t18523 + t18527 + t18529 + t18531;
    (t18521, t18523, t18527, t18529, t18531, t18532)
}
