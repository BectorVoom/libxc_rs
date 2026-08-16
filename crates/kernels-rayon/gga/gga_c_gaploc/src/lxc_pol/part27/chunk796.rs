//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 796/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk796(t4598: f64, t973: f64, t1628: f64, t2704: f64, t2710: f64, t1589: f64, t2586: f64, t2027: f64, t2628: f64, t6138: f64, t959: f64, t5673: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7545 = t4598 * t973;
    let t7550 = t1628 * t2704;
    let t7553 = t1628 * t2710;
    let t7558 = t1589 * t2586;
    let t7563 = t2027 * t2628;
    let t7565 = t6138 * t959;
    let t7567 = t5673 * t959;
    (t7545, t7550, t7553, t7558, t7563, t7565, t7567)
}
