//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 916/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk916(t11326: f64, t8621: f64, t3060: f64, t3708: f64) -> (f64, f64) {
    let t11327 = t11326 * t8621;
    let t11329 = t3060 * t3708;
    (t11327, t11329)
}
