//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 773/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk773<F: Float>(t3259: F, t6350: F, t3257: F, t274: F, t745: F, t2084: F, t2157: F, t2189: F, t2346: F, t3235: F, t1: F, t2298: F) -> (F, F, F, F, F, F) {
    let t6351 = t6350 * t3259;
    let t6352 = t3257 * t6351;
    let t6355 = t745 * t274;
    let t6356 = t2084 * t6355;
    let t6357 = t3257 * t6356;
    let t6360 = t2157 * t2189;
    let t6362 = t3235 * t2346 * t6360;
    let t6365 = t2298 * t1;
    (t6352, t6355, t6357, t6360, t6362, t6365)
}
