//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 502/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk502<F: Float>(t2190: F, t858: F, t867: F, t866: F, t2156: F, t343: F) -> (F, F, F, F) {
    let t2191 = t858 * t2190;
    let t2192 = t867 * t2191;
    let t2194 = t866 * t2192 / 96.0;
    let t2195 = t2156 * t343;
    (t2191, t2192, t2194, t2195)
}
