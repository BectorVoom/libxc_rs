//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1135/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1135(t11849: f64, t1952: f64, t919: f64, t11761: f64, t34005: f64, t3775: f64, t9586: f64, t11428: f64, t667: f64, t3326: f64, t29576: f64, t29582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34033 = t11849 * t1952 * t919;
    let t34036 = t34005 * t11761;
    let t34038 = t3775 * t9586;
    let t34040 = t667 * t11428;
    let t34041 = t34040 * t3326;
    let t34043 = t29576 * t34041 * t29582;
    (t34033, t34036, t34038, t34040, t34041, t34043)
}
