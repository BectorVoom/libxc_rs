//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 965/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk965(t2367: f64, t3285: f64, t1220: f64, t3289: f64, t4282: f64, t9044: f64, t3245: f64, t4290: f64, t4289: f64, t1186: f64, t2908: f64, t2910: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9233 = t2367 * t3285;
    let t9234 = t1220 * t9233;
    let t9236 = t2367 * t3289;
    let t9237 = t1220 * t9236;
    let t9240 = t4282 * t9044;
    let t9241 = t3245 * t9240;
    let t9243 = t4290 * t9044;
    let t9244 = t4289 * t9243;
    let t9251 = t2908 * t1186;
    let t9254 = 1.0_f64 / t2910 / t474;
    (t9233, t9234, t9236, t9237, t9240, t9241, t9243, t9244, t9251, t9254)
}
