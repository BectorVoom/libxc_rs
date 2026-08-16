//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 901/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk901(t6509: f64, t883: f64, t4782: f64, t9272: f64, t123: f64, t2293: f64, t912: f64, t587: f64, t544: f64, t6603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9273 = t883 * t6509;
    let t9274 = t4782 * t9273;
    let t9276 = 0.11502877786176224903e1_f64 * t9272 * t9274;
    let t9277 = t2293 * t123;
    let t9278 = t9277 * t883;
    let t9279 = t912 * t9278;
    let t9280 = t587 * t9279;
    let t9281 = 0.38342925953920749676e0_f64 * t9280;
    let t9285 = t544 * t6603;
    (t9274, t9276, t9278, t9279, t9281, t9285)
}
