//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 797/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk797<F: Float>(t305: F, t6072: F, t296: F, t413: F, t2092: F, t2096: F, t2100: F, t817: F, t2106: F, t814: F, t816: F, t322: F) -> (F, F, F, F, F) {
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = F::cast_from(0.47400060215270560269e0_f64) * t6075;
    let t6080 = t2092 * t2096;
    let t6086 = t2100 * t817;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = F::cast_from(1.0_f64) / t6094;
    let t6096 = t322 * t6095;
    (t6076, t6080, t6086, t6089, t6096)
}
