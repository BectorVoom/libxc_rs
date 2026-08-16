//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 860/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk860(t41749: f64, t6717: f64, t6914: f64, t40449: f64, t40452: f64, t10608: f64, t9272: f64, t9278: f64, t1445: f64, t26809: f64, t3085: f64, t4527: f64) -> (f64, f64, f64, f64, f64) {
    let t42315 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t41749;
    let t42340 = 0.63904876589867916127e-1_f64 * t40449;
    let t42341 = 0.31952438294933958063e0_f64 * t40452;
    let t42349 = t9272 * t10608 * t9278;
    let t42350 = 0.11502877786176224903e1_f64 * t42349;
    let t42354 = 0.27606906686822939767e2_f64 * t4527 * t1445 * t26809 * t3085;
    (t42315, t42340, t42341, t42350, t42354)
}
