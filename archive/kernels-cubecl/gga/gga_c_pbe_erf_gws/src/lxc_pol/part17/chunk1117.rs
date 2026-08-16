//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1117/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1117<F: Float>(t14091: F, t14093: F, t3139: F, t6409: F, t4028: F, t331: F, t911: F, t56: F, t863: F) -> (F, F, F, F, F) {
    let t14094 = t14091 * t14093;
    let t14096 = t3139 * t6409;
    let t14097 = t4028 * t14096;
    let t14099 = t911 * t331;
    let t14101 = t863 * t14099 * t56;
    (t14094, t14096, t14097, t14099, t14101)
}
