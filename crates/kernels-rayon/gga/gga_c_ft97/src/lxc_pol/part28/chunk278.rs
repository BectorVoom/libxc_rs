//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 278/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk278(t1710: f64, t938: f64, t428: f64, t1725: f64, t935: f64, t173: f64, t934: f64, t419: f64, t1736: f64, t420: f64, t2984: f64, t1527: f64, t2993: f64) -> (f64, f64, f64, f64, f64) {
    let t3077 = t1710 * t938;
    let t3078 = t3077 * t428;
    let t3083 = t1725 * t935;
    let t3085 = t173 * t934;
    let t3086 = t419 * t3085;
    let t3088 = t420 * t1736;
    let t3089 = t3088 * t2984;
    let t3090 = t419 * t3089;
    let t3092 = t1527 * t2993;
    (t3078, t3083, t3086, t3090, t3092)
}
