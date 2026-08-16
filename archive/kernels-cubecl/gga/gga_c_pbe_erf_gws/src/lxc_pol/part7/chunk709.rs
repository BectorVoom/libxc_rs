//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 709/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk709<F: Float>(t10: F, t5798: F, t1: F, t1563: F, t501: F, t119: F, t1504: F, t155: F, t331: F, t481: F, t1557: F, t128: F, t485: F) -> (F, F, F, F, F, F, F) {
    let t5799 = t10 * t5798;
    let t5803 = t501 * t1563 * t1;
    let t5805 = t119 * t155 * t1504;
    let t5806 = t5803 * t5805;
    let t5809 = t119 * t331 * t481;
    let t5810 = t1557 * t5809;
    let t5813 = t485 * t128 * t1;
    (t5799, t5803, t5805, t5806, t5809, t5810, t5813)
}
