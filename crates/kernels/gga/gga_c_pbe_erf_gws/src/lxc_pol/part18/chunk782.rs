//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 782/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk782<F: Float>(t155: F, t506: F, t2911: F, t2913: F, t133: F, t8146: F, t8199: F, t2522: F, t285: F, t545: F, t281: F, t1368: F, t991: F, t169: F, t2848: F, t301: F, t784: F) -> (F, F, F, F, F, F) {
    let t8236 = t155 * t506;
    let t8238 = t2911 * t8236 * t2913;
    let t8249 = 0.11495033333333333333e1 * t133 * t8146;
    let t8252 = t133 * t8199;
    let t8265 = t2522 * t545 * t285;
    let t8267 = 0.23948468020509218188e-1 * t281 * t8265;
    let t8269 = t991 * t1368 * t285;
    let t8270 = t281 * t8269;
    let t8275 = 0.10809180959278284142e0 * t169 * t784 * t2848 * t301;
    (t8238, t8249, t8252, t8267, t8270, t8275)
}
