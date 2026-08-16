//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1241/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1241(t116: f64, t1968: f64, t204: f64, t34159: f64, t169: f64, t3081: f64, t35194: f64, t11412: f64, t26447: f64, t27624: f64, t11431: f64, t27754: f64) -> (f64, f64, f64, f64) {
    let t35316 = t116 * t1968 * t34159 * t204;
    let t35319 = t169 * t35194 * t3081;
    let t35323 = t169 * t11412 * t26447 * t27624;
    let t35325 = t11431 * t27754;
    (t35316, t35319, t35323, t35325)
}
