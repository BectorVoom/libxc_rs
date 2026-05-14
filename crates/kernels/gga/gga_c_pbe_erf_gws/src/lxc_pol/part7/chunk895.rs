//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 895/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk895<F: Float>(t16806: F, t16811: F, t16814: F, t16818: F, t16821: F, t16829: F, t16832: F, t16834: F, t16838: F, t16842: F, t18192: F, t18196: F, t19: F, t336: F, t4562: F, t714: F) -> (F, F) {
    let t18197 = -t16806 - t16811 + t16814 - t16818 + t16821 + t16829 + t16832 + 32.0 / 81.0 * t18192 + t18196 - t16834 + t16838 - t16842;
    let t18201 = t4562 * t19 * t336 * t714;
    (t18197, t18201)
}
