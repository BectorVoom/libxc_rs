//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 698/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk698<F: Float>(t481: F, t510: F, t5651: F, t142: F, t1533: F, t525: F, t2030: F, t520: F, t2032: F, t1452: F, t169: F, t301: F, t784: F) -> (F, F, F, F, F, F, F) {
    let t5652 = t510 * t481;
    let t5653 = t5651 * t5652;
    let t5656 = t142 * t1533;
    let t5657 = t525 * t5656;
    let t5660 = t2030 * t520;
    let t5661 = t5660 * t2032;
    let t5666 = t169 * t784 * t1452 * t301;
    (t5652, t5653, t5656, t5657, t5660, t5661, t5666)
}
