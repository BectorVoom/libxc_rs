//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 789/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk789(t10608: f64, t9272: f64, t9278: f64, t34600: f64, t544: f64, t9287: f64, t34604: f64, t12938: f64, t2464: f64, t587: f64, t40517: f64, t26435: f64, t6710: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42349 = t9272 * t10608 * t9278;
    let t42366 = t544 * t34600 * t9287;
    let t42369 = t544 * t34604 * t9287;
    let t42378 = t587 * t2464 * t12938;
    let t42381 = 0.25561950635947166451e0_f64 * t40517;
    let t42400 = t6710 * t9438 * t26435;
    (t42349, t42366, t42369, t42378, t42381, t42400)
}
