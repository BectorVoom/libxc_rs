//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1280/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1280(t22474: f64, t22162: f64, t22164: f64, t22167: f64, t22169: f64, t22171: f64, t22175: f64, t22184: f64, t22188: f64, t22313: f64, t22355: f64, t22359: f64, t22361: f64, t22363: f64, t22366: f64, t22374: f64, t22376: f64, t22378: f64, t22380: f64, t22382: f64, t22385: f64) -> (f64, f64) {
    let t22475 = 0.28582678745379824648e-3_f64 * t22474;
    let t22476 = t22162 + t22164 + t22167 - t22169 + t22171 + t22175 - t22184 - t22188 - t22313 + t22355 + t22359 + t22361 - t22363 - t22366 - t22374 - t22376 - t22378 + t22380 + t22382 - t22385;
    (t22475, t22476)
}
