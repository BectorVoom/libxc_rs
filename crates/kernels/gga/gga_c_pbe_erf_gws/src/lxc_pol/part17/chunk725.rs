//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 725/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk725<F: Float>(t2228: F, t840: F, t2367: F, t2373: F, t2306: F, t2365: F, t2382: F, t833: F, t2222: F, t814: F, t898: F, t938: F) -> (F, F, F, F, F, F) {
    let t4467 = t840 * t2228;
    let t4469 = t2367 * t2373;
    let t4473 = t2306 * t2365;
    let t4474 = t2382 * t4473;
    let t4475 = t4474 * t833;
    let t4477 = t840 * t2222;
    let t4482 = t898 * t814 * t938;
    (t4467, t4469, t4474, t4475, t4477, t4482)
}
