//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1196/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1196<F: Float>(t3969: F, t916: F, t2250: F, t14024: F, t2129: F, t2153: F, t899: F, t923: F, t2348: F, t56: F, t837: F, t863: F, t911: F) -> (F, F, F, F, F, F) {
    let t51350 = t3969 * t916;
    let t51351 = t2250 * t51350;
    let t51358 = t2129 * t14024;
    let t51371 = t899 * t2153 * t923;
    let t51372 = t51371 * t2348;
    let t51382 = t863 * t911 * t837 * t56;
    (t51350, t51351, t51358, t51371, t51372, t51382)
}
