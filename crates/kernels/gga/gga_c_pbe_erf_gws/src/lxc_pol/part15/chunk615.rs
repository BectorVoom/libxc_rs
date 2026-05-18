//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 615/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk615<F: Float>(t513: F, t981: F, t520: F, t985: F, t133: F, t2878: F, t119: F, t132: F, t506: F, t9: F, t481: F, t967: F) -> (F, F, F, F, F, F) {
    let t2902 = t981 * t513;
    let t2905 = t985 * t520;
    let t2909 = t133 * t2878;
    let t2911 = t132 * t119;
    let t2912 = t9 * t506;
    let t2913 = t967 * t481;
    (t2902, t2905, t2909, t2911, t2912, t2913)
}
