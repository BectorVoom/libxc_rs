//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 818/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk818(t144: f64, t1736: f64, t5526: f64, t674: f64, t8893: f64, t5542: f64, t5708: f64, t5211: f64, t5713: f64, t2993: f64, t3127: f64, t5392: f64) -> (f64, f64, f64, f64, f64) {
    let t9323 = t1736 * t144;
    let t9325 = t9323 * t674 * t5526;
    let t9326 = t8893 * t9325;
    let t9328 = t5708 * t5542;
    let t9330 = t5211 * t144 * t5713;
    let t9331 = t9328 * t9330;
    let t9333 = t2993 * t3127;
    let t9334 = t9333 * t5392;
    (t9325, t9326, t9330, t9331, t9334)
}
