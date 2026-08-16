//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 654/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk654(t442: f64, t5216: f64, t5215: f64, t505: f64, t674: f64, t172: f64) -> (f64, f64, f64, f64) {
    let t5217 = t5216 * t442;
    let t5218 = t5215 * t5217;
    let t5247 = t505 * t674;
    let t5248 = t5247 * t172;
    (t5217, t5218, t5247, t5248)
}
