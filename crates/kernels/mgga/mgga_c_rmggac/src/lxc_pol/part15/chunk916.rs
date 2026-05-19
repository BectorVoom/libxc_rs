//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 916/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk916<F: Float>(t7717: F, t9783: F, t39277: F, t9123: F, t39234: F, t39250: F, t39252: F, t39256: F, t39265: F, t39286: F, t39290: F, t45277: F, t45283: F, t45285: F, t45289: F, t45291: F, t45293: F, t45295: F, t45300: F, t45305: F) -> F {
    let t45307 = t7717 * t9783;
    let t45309 = t39277 * t9123;
    let t45311 = F::cast_from(0.1064114997332445985e-4_f64) * t45277 + F::cast_from(0.53205749866622299248e-5_f64) * t45283 - F::cast_from(0.85129199786595678796e-5_f64) * t45285 - t39234 - F::cast_from(0.59590439850616975158e-4_f64) * t39250 + F::cast_from(0.59590439850616975158e-4_f64) * t39252 + F::cast_from(0.27274661654245341728e-1_f64) * t45289 - F::cast_from(0.20455996240684006297e-1_f64) * t45291 - t39256 + t39265 + F::cast_from(0.17025839957319135759e-4_f64) * t45293 - F::cast_from(0.25538759935978703639e-4_f64) * t45295 + t39286 - t39290 + F::cast_from(0.15961724959986689774e-4_f64) * t45300 + F::cast_from(0.53205749866622299248e-5_f64) * t45305 - F::cast_from(0.53205749866622299248e-5_f64) * t45307 + F::cast_from(0.1064114997332445985e-4_f64) * t45309;
    t45311
}
