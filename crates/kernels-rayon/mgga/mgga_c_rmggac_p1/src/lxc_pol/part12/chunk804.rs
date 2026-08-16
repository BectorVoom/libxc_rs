//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 804/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk804(t8498: f64, t8505: f64, t8509: f64, t8513: f64, t8523: f64, t8527: f64, t8529: f64, t34567: f64, t7391: f64, t7395: f64, t7398: f64, t7401: f64, t8538: f64, t9335: f64, t9336: f64, t9337: f64, t9768: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38235 = 0.85129199786595678796e-5_f64 * t8498;
    let t38236 = 0.25538759935978703638e-4_f64 * t8505;
    let t38237 = 0.76616279807936110914e-4_f64 * t8509;
    let t38238 = 0.85129199786595678796e-5_f64 * t8513;
    let t38239 = 0.20455996240684006296e-1_f64 * t8523;
    let t38240 = 0.20455996240684006296e-1_f64 * t8527;
    let t38242 = 0.27274661654245341728e-1_f64 * t8529;
    let t38246 = -t38242 + t9768 - 0.54549323308490683456e-1_f64 * t8538 - t34567 + 0.86737941314158990624e-4_f64 * t7391 + 0.86737941314158990624e-4_f64 * t7395 - t7398 - t7401 + t9335 + t9336 - t9337;
    (t38235, t38236, t38237, t38238, t38239, t38240, t38246)
}
