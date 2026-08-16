//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 713/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk713<F: Float>(t2376: F, t2409: F, t4207: F, t1144: F, t1206: F, t338: F, t1161: F, t1205: F) -> (F, F, F) {
    let t4209 = t2409 * t2376 * t4207;
    let t4212 = t1144 * t1206;
    let t4213 = t338 * t4212;
    let t4216 = t1205 * t1161;
    (t4209, t4213, t4216)
}
