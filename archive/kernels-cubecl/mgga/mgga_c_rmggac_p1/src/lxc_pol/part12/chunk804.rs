//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 804/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk804<F: Float>(t8498: F, t8505: F, t8509: F, t8513: F, t8523: F, t8527: F, t8529: F, t34567: F, t7391: F, t7395: F, t7398: F, t7401: F, t8538: F, t9335: F, t9336: F, t9337: F, t9768: F) -> (F, F, F, F, F, F, F) {
    let t38235 = F::cast_from(0.85129199786595678796e-5_f64) * t8498;
    let t38236 = F::cast_from(0.25538759935978703638e-4_f64) * t8505;
    let t38237 = F::cast_from(0.76616279807936110914e-4_f64) * t8509;
    let t38238 = F::cast_from(0.85129199786595678796e-5_f64) * t8513;
    let t38239 = F::cast_from(0.20455996240684006296e-1_f64) * t8523;
    let t38240 = F::cast_from(0.20455996240684006296e-1_f64) * t8527;
    let t38242 = F::cast_from(0.27274661654245341728e-1_f64) * t8529;
    let t38246 = -t38242 + t9768 - F::cast_from(0.54549323308490683456e-1_f64) * t8538 - t34567 + F::cast_from(0.86737941314158990624e-4_f64) * t7391 + F::cast_from(0.86737941314158990624e-4_f64) * t7395 - t7398 - t7401 + t9335 + t9336 - t9337;
    (t38235, t38236, t38237, t38238, t38239, t38240, t38246)
}
