//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 952/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk952(t5929: f64, t5933: f64, t5936: f64, t7498: f64, t7504: f64, t7509: f64, t7513: f64, t7518: f64, t7519: f64, t7524: f64, t7526: f64, t7529: f64, t7532: f64, t7536: f64, t7538: f64, t7540: f64, t7563: f64) -> f64 {
    let t8432 = -t7498 - t7504 + t7509 - t7513 + t7518 + t7519 + t7524 + t7526 - t7529 - t7532 - t7536 - t7538 + t7540 + t7563 + t5929 + t5933 + 0.10821041362364843377e0_f64 * t5936;
    t8432
}
