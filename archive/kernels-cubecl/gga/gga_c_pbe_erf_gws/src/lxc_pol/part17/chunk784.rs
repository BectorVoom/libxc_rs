//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 784/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk784<F: Float>(t5803: F, t5805: F, t119: F, t331: F, t481: F, t1557: F, t1: F, t128: F, t485: F, t1513: F, t1544: F, t156: F) -> (F, F, F, F, F) {
    let t5806 = t5803 * t5805;
    let t5809 = t119 * t331 * t481;
    let t5810 = t1557 * t5809;
    let t5813 = t485 * t128 * t1;
    let t5814 = t5813 * t5805;
    let t5816 = t1513 * t5809;
    let t5818 = t156 * t1544;
    (t5806, t5810, t5814, t5816, t5818)
}
