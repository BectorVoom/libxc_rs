//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 735/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk735<F: Float>(t2259: F, t6402: F, t2255: F, t2279: F, t6350: F, t343: F, t6269: F, t337: F, t2121: F, t2134: F, t2115: F, t2142: F, t2276: F, t6401: F) -> (F, F, F, F, F, F, F) {
    let t6403 = t6402 * t2259;
    let t6406 = t2255 * t6350 * t2279;
    let t6409 = t6269 * t343;
    let t6410 = t337 * t6409;
    let t6411 = t2121 * t6410;
    let t6413 = t2134 * t6411 / 32.0;
    let t6414 = t2115 * t2142;
    let t6415 = 7.0 / 96.0 * t6414;
    let t6416 = t2276 * t6401;
    (t6403, t6406, t6410, t6411, t6413, t6415, t6416)
}
