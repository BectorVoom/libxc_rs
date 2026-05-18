//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 985/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk985<F: Float>(t10306: F, t10351: F, t10414: F, t10469: F, t10601: F, t10662: F, t10707: F, t10752: F, t10860: F, t10882: F, t10905: F, t10953: F, t11003: F, t11035: F, t11107: F, t11147: F) -> F {
    let t11151 = t10306 + t10351 + t10414 + t10469 + t10601 + t10662 + t10707 + t10752 + t10860 + t10882 + t10905 + t10953 + t11003 + t11035 + t11107 + t11147;
    t11151
}
