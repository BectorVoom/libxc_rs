//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 896/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk896(t167: f64, t34918: f64, t574: f64, t1391: f64, t6615: f64, t3578: f64, t7407: f64, t144: f64, t7357: f64, t925: f64, t9144: f64, t1053: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35181 = t574 * t167 * t34918;
    let t35185 = t574 * t1391 * t6615;
    let t35188 = t3578 * t7407;
    let t35189 = t144 * t35188;
    let t35192 = t7357 * t925;
    let t35193 = t9144 * t35192;
    let t35196 = t7407 * t1053;
    (t35181, t35185, t35188, t35189, t35192, t35193, t35196)
}
