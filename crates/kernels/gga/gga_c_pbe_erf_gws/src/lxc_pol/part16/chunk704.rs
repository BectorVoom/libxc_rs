//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 704/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk704<F: Float>(t2376: F, t2409: F, t4088: F, t1206: F, t892: F, t338: F, t1205: F, t938: F) -> (F, F, F) {
    let t4090 = t2409 * t2376 * t4088;
    let t4093 = t892 * t1206;
    let t4094 = t338 * t4093;
    let t4097 = t1205 * t938;
    (t4090, t4094, t4097)
}
