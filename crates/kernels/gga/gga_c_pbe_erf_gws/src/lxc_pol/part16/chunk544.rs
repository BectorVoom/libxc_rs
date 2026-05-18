//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 544/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk544<F: Float>(t2332: F, t369: F, t371: F, t364: F, t2112: F, t824: F, t905: F, t367: F, t899: F, t912: F) -> (F, F, F, F, F, F, F) {
    let t2333 = t2332 * t369;
    let t2334 = t2333 * t371;
    let t2336 = F::new(119.0) / F::new(13824.0) * t364 * t2334;
    let t2337 = param_a_c * t2112;
    let t2338 = t2337 * t824;
    let t2339 = t905 * t2338;
    let t2343 = t899 * t912 * t367;
    (t2333, t2334, t2336, t2337, t2338, t2339, t2343)
}
