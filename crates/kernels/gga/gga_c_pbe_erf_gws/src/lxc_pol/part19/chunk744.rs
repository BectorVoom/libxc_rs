//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 744/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk744<F: Float>(t6055: F, t6056: F, t1492: F, t751: F, t1497: F, t309: F, t310: F, t311: F, t305: F, t296: F, t413: F, t2092: F, t2096: F, t2106: F, t814: F, t816: F) -> (F, F, F, F, F, F, F) {
    let t6058 = 0.45692190944741466895e-5 * t6055 * t6056;
    let t6061 = t751 * t1492;
    let t6064 = 0.59871170051273045469e-1 * t751 * t1497;
    let t6072 = 1.0 / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = 0.47400060215270560269e0 * t6075;
    let t6080 = t2092 * t2096;
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    (t6058, t6061, t6064, t6076, t6080, t6089, t6094)
}
