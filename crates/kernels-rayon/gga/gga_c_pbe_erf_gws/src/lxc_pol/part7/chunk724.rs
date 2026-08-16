//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 724/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk724(t2004: f64, t5953: f64, t5356: f64, t5359: f64, t5375: f64, t5377: f64, t5381: f64, t5397: f64, t5405: f64, t5933: f64, t5936: f64, t5938: f64, t5940: f64, t5944: f64, t5945: f64, t5948: f64, t5949: f64, t5952: f64) -> f64 {
    let t5954 = t5953 * t2004;
    let t5956 = t5933 + 0.32463124087094530131e0_f64 * t5936 + 0.64926248174189060262e0_f64 * t5938 + 0.21642082724729686754e0_f64 * t5940 - t5944 - t5356 + t5359 + t5375 + 8.0_f64 * t5945 + t5948 + 4.0_f64 * t5949 + t5952 - t5377 + t5381 + 0.33545228223331014468e-1_f64 * t5954 + t5397 + t5405;
    t5956
}
