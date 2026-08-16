//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk952(t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t17399: f64, t17401: f64, t17403: f64, t17406: f64, t17409: f64, t17412: f64, t17419: f64, t8728: f64) -> f64 {
    let t17499 = -t8728 - 0.60384999999999999999e0_f64 * t17346 + 0.181155e1_f64 * t17354 - 0.3883875e1_f64 * t17399 + 0.247573125e0_f64 * t17401 + 0.16504875e0_f64 * t17403 + 0.16557e0_f64 * t17406 - 0.49671e0_f64 * t17409 - 0.36793333333333333333e-1_f64 * t17412 - 0.33547222222222222222e0_f64 * t17338 + 0.12077e1_f64 * t17342 - 0.181155e1_f64 * t17350 - 0.301925e0_f64 * t17358 - 0.82785e-1_f64 * t17419;
    t17499
}
