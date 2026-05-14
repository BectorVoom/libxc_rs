//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 449/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk449<F: Float>(t339: F, t816: F, t19: F, t793: F, t796: F, t801: F, t116: F, t299: F, t799: F, t798: F, t814: F, t817: F, t274: F, t322: F, t2083: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2085 = t816 * t339;
    let t2092 = t793 * t796 * t19;
    let t2093 = t2092 * t801;
    let t2096 = t799 * t299 * t116;
    let t2097 = t798 * t2096;
    let t2098 = 0.6846054806677777778e0 * t2097;
    let t2102 = t814 * t817;
    let t2105 = t816 * t274;
    let t2106 = 1.0 / t2105;
    let t2107 = t322 * t2106;
    let t2118 = t2083 * t274;
    (t2085, t2092, t2093, t2096, t2098, t2102, t2105, t2106, t2107, t2118)
}
