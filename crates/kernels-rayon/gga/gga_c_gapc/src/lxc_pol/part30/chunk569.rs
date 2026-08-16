//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 569/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk569(t2701: f64, t646: f64, t918: f64, t3343: f64, t1026: f64, t933: f64, t937: f64, t1081: f64, t954: f64, t969: f64, t1936: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3345 = t646 * t918 * t2701;
    let t3346 = t3343 * t3345;
    let t3348 = t933 * t1026;
    let t3349 = t3348 * t937;
    let t3351 = t1081 * t954;
    let t3355 = t1081 * t969;
    let t3357 = t325 * t1936;
    (t3345, t3346, t3348, t3349, t3351, t3355, t3357)
}
