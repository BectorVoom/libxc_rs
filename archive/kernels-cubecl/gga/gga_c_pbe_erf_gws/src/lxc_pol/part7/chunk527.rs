//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 527/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk527<F: Float>(t191: F, t2331: F, t369: F, t371: F, t364: F, t2112: F, param_a_c: F) -> (F, F, F) {
    let t2332 = t2331 * t191;
    let t2333 = t2332 * t369;
    let t2334 = t2333 * t371;
    let t2336 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t364 * t2334;
    let t2337 = param_a_c * t2112;
    (t2332, t2336, t2337)
}
