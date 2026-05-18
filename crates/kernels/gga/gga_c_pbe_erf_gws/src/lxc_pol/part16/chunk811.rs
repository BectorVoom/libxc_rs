//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 811/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk811<F: Float>(t337: F, t6409: F, t2121: F, t2115: F, t2142: F, t2276: F, t6401: F, t2281: F, t2100: F, t369: F, t814: F, t931: F) -> (F, F, F, F, F, F) {
    let t6410 = t337 * t6409;
    let t6411 = t2121 * t6410;
    let t6414 = t2115 * t2142;
    let t6416 = t2276 * t6401;
    let t6417 = t6416 * t2281;
    let t6421 = t2100 * t369;
    let t6424 = t814 * t931;
    (t6411, t6414, t6416, t6417, t6421, t6424)
}
