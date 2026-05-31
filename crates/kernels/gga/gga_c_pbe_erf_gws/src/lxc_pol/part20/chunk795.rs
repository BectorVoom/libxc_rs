//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 795/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk795<F: Float>(t6055: F, t6056: F, t1492: F, t751: F, t1497: F, t309: F, t310: F, t311: F, t305: F, t296: F, t413: F, t2092: F, t2096: F) -> (F, F, F, F, F) {
    let t6058 = F::cast_from(0.45692190944741466895e-5_f64) * t6055 * t6056;
    let t6061 = t751 * t1492;
    let t6064 = F::cast_from(0.59871170051273045469e-1_f64) * t751 * t1497;
    let t6072 = F::cast_from(1.0_f64) / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = F::cast_from(0.47400060215270560269e0_f64) * t6075;
    let t6080 = t2092 * t2096;
    (t6058, t6061, t6064, t6076, t6080)
}
