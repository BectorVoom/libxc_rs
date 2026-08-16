//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 916/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk916(t7717: f64, t9783: f64, t39277: f64, t9123: f64, t39234: f64, t39250: f64, t39252: f64, t39256: f64, t39265: f64, t39286: f64, t39290: f64, t45277: f64, t45283: f64, t45285: f64, t45289: f64, t45291: f64, t45293: f64, t45295: f64, t45300: f64, t45305: f64) -> f64 {
    let t45307 = t7717 * t9783;
    let t45309 = t39277 * t9123;
    let t45311 = 0.1064114997332445985e-4_f64 * t45277 + 0.53205749866622299248e-5_f64 * t45283 - 0.85129199786595678796e-5_f64 * t45285 - t39234 - 0.59590439850616975158e-4_f64 * t39250 + 0.59590439850616975158e-4_f64 * t39252 + 0.27274661654245341728e-1_f64 * t45289 - 0.20455996240684006297e-1_f64 * t45291 - t39256 + t39265 + 0.17025839957319135759e-4_f64 * t45293 - 0.25538759935978703639e-4_f64 * t45295 + t39286 - t39290 + 0.15961724959986689774e-4_f64 * t45300 + 0.53205749866622299248e-5_f64 * t45305 - 0.53205749866622299248e-5_f64 * t45307 + 0.1064114997332445985e-4_f64 * t45309;
    t45311
}
