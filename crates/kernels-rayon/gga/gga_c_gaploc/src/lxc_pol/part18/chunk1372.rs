//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1372/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1372(t204: f64, t34378: f64, t587: f64, t10421: f64, t21417: f64, t30374: f64, t30378: f64, t30380: f64, t30382: f64, t34352: f64, t34354: f64, t34356: f64, t34358: f64, t34361: f64, t34366: f64, t34370: f64, t34374: f64, t34377: f64) -> f64 {
    let t34381 = 0.18404604457881959845e2_f64 * t587 * t204 * t34378;
    let t34382 = t10421 * t21417;
    let t34383 = 0.59584149919750711116e-1_f64 * t34382;
    let t34384 = t34352 + t34354 + t34356 + t34358 + t34361 + t34366 - t34370 + t34374 + t34377 - t34381 + t30374 - t30378 + t34383 - t30380 - t30382;
    t34384
}
