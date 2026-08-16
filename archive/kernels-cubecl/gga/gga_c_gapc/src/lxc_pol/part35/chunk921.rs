//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 921/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk921<F: Float>(t11304: F, t11306: F, t11309: F, t11314: F, t11318: F, t11323: F, t11327: F, t11330: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F, t11358: F, t11363: F, t11367: F, t11369: F) -> F {
    let t11371 = F::cast_from(0.90579542097823505428e-7_f64) * t11304 - F::cast_from(0.90579542097823505428e-7_f64) * t11306 + F::cast_from(0.33148893438893365995e-7_f64) * t11309 + F::cast_from(0.17376185052903442709e-3_f64) * t11314 + F::cast_from(0.17376185052903442709e-3_f64) * t11318 - F::cast_from(0.25745714186718600948e-5_f64) * t11323 - F::cast_from(0.35172068325509175607e-8_f64) * t11327 + F::cast_from(0.33148893438893365995e-7_f64) * t11330 - F::cast_from(0.12670134934408760309e-3_f64) * t11334 - F::cast_from(0.12650960286458333334e-5_f64) * t11337 - F::cast_from(0.20241536458333333334e-4_f64) * t11339 + F::cast_from(0.12229261610243055556e-4_f64) * t11345 - F::cast_from(0.17376185052903442709e-3_f64) * t11348 - F::cast_from(0.20241536458333333334e-4_f64) * t11351 + F::cast_from(0.54106179813099907243e-4_f64) * t11353 - F::cast_from(0.21103240995305505364e-7_f64) * t11358 + F::cast_from(0.39292488356234936494e-8_f64) * t11363 - F::cast_from(0.26419033111865189083e-7_f64) * t11367 - F::cast_from(0.6629778687778673199e-7_f64) * t11369;
    t11371
}
