//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1126/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1126<F: Float>(t6180: F, t6188: F, t2189: F, t343: F, t745: F, t2121: F, t2122: F, t337: F, t2382: F, t6566: F, t20189: F, t2137: F) -> (F, F, F, F) {
    let t20234 = t6188 * t6180 / F::new(16.0);
    let t20236 = t745 * t2189 * t343;
    let t20244 = t2121 * t337 * t2122 * t745;
    let t20246 = F::new(7.0) / F::new(48.0) * t2382 * t6566 * t20244;
    let t20247 = t20189 * t2137;
    (t20234, t20236, t20246, t20247)
}
