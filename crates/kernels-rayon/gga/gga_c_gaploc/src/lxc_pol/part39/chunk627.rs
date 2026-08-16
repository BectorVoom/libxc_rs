//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 627/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk627(t10409: f64, t9263: f64, t9422: f64, t10381: f64, t10384: f64, t10387: f64, t10388: f64, t10394: f64, t10395: f64, t10398: f64, t10401: f64, t10404: f64, t10406: f64, t1537: f64, t567: f64, t9363: f64, t9366: f64, t9370: f64) -> (f64, f64, f64) {
    let t10410 = t9263 * t10409;
    let t10411 = 0.38342925953920749676e0_f64 * t10410;
    let t10412 = 0.63904876589867916128e-1_f64 * t9422;
    let t10413 = t10381 + t10384 + t10387 + 0.23005755572352449806e1_f64 * t567 * t10388 + t9363 + t9366 - t9370 + t10394 - t10395 - t10398 + t10401 - t10404 - 0.51123901271894332902e0_f64 * t1537 * t10406 - t10411 + t10412;
    (t10411, t10412, t10413)
}
