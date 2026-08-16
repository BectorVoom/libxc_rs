//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1077/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1077(t11831: f64, t33338: f64, t11764: f64, t920: f64, t2648: f64, t3769: f64, t11834: f64, t16403: f64, t7191: f64, t1026: f64, t2674: f64, t9827: f64) -> (f64, f64, f64, f64, f64) {
    let t33339 = t33338 * t11831;
    let t33341 = t11764 * t920;
    let t33343 = t3769 * t2648;
    let t33346 = t11834 * t16403 * t7191;
    let t33349 = t2674 * t1026 * t9827;
    (t33339, t33341, t33343, t33346, t33349)
}
