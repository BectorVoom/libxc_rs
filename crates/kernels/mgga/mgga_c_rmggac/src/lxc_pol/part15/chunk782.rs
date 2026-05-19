//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 782/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk782<F: Float>(t8444: F, t8448: F, t8452: F, t8460: F, t8494: F, t8498: F, t8505: F, t8509: F, t8513: F, t8523: F, t8527: F, t8529: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38218 = F::cast_from(0.85129199786595678796e-5_f64) * t8444;
    let t38219 = F::cast_from(0.85129199786595678796e-5_f64) * t8448;
    let t38220 = F::cast_from(0.85129199786595678796e-5_f64) * t8452;
    let t38221 = F::cast_from(0.39914139006212695214e-1_f64) * t8460;
    let t38234 = F::cast_from(0.85129199786595678796e-5_f64) * t8494;
    let t38235 = F::cast_from(0.85129199786595678796e-5_f64) * t8498;
    let t38236 = F::cast_from(0.25538759935978703638e-4_f64) * t8505;
    let t38237 = F::cast_from(0.76616279807936110914e-4_f64) * t8509;
    let t38238 = F::cast_from(0.85129199786595678796e-5_f64) * t8513;
    let t38239 = F::cast_from(0.20455996240684006296e-1_f64) * t8523;
    let t38240 = F::cast_from(0.20455996240684006296e-1_f64) * t8527;
    let t38242 = F::cast_from(0.27274661654245341728e-1_f64) * t8529;
    (t38218, t38219, t38220, t38221, t38234, t38235, t38236, t38237, t38238, t38239, t38240, t38242)
}
