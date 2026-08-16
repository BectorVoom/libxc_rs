//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 750/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk750(t455: f64, t4623: f64, t1231: f64, t440: f64, t441: f64, t1257: f64, t67: f64, t62: f64, t1261: f64, t1314: f64, t457: f64, t1253: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4624 = t4623 * t455;
    let t4630 = t1231 * t440;
    let t4631 = t4630 * t441;
    let t4635 = 1.0_f64 / t1257 / t67;
    let t4636 = t62 * t4635;
    let t4637 = t4630 * t1261;
    let t4640 = t457 * t1314;
    let t4643 = t1253 * t1261;
    (t4624, t4630, t4631, t4636, t4637, t4640, t4643)
}
