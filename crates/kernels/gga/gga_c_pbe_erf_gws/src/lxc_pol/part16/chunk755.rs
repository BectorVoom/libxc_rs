//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 755/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk755<F: Float>(t1721: F, t401: F, t1715: F, t205: F, t626: F, t191: F, t1641: F, t261: F, t174: F, t838: F, t1243: F, t628: F) -> (F, F, F, F, F, F, F) {
    let t5054 = t401 * t1721;
    let t5056 = t401 * t1715;
    let t5060 = F::new(1.0) / t205 / t626;
    let t5061 = t191 * t5060;
    let t5063 = F::new(1.0) / t1641 / t261;
    let t5081 = t174 * t838 * t205;
    let t5082 = F::new(0.11197407407407407407e0) * t5081;
    let t5083 = t1243 * t628;
    (t5054, t5056, t5061, t5063, t5081, t5082, t5083)
}
