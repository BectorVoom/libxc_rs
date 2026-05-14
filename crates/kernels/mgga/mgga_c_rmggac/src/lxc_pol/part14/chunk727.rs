//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 727/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk727<F: Float>(t8494: F, t8498: F, t8505: F, t8509: F, t8513: F, t8523: F, t8527: F, t8529: F, t34567: F, t7391: F, t7395: F, t7398: F, t7401: F, t8538: F, t9335: F, t9336: F, t9337: F, t9768: F) -> (F, F, F, F, F, F, F, F) {
    let t38234 = 0.85129199786595678796e-5 * t8494;
    let t38235 = 0.85129199786595678796e-5 * t8498;
    let t38236 = 0.25538759935978703638e-4 * t8505;
    let t38237 = 0.76616279807936110914e-4 * t8509;
    let t38238 = 0.85129199786595678796e-5 * t8513;
    let t38239 = 0.20455996240684006296e-1 * t8523;
    let t38240 = 0.20455996240684006296e-1 * t8527;
    let t38242 = 0.27274661654245341728e-1 * t8529;
    let t38246 = -t38242 + t9768 - 0.54549323308490683456e-1 * t8538 - t34567 + 0.86737941314158990624e-4 * t7391 + 0.86737941314158990624e-4 * t7395 - t7398 - t7401 + t9335 + t9336 - t9337;
    (t38234, t38235, t38236, t38237, t38238, t38239, t38240, t38246)
}
