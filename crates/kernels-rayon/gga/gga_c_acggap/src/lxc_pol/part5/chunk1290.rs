//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1290/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1290(t1856: f64, t3237: f64, t1008: f64, t6200: f64, t1095: f64, t322: f64, t384: f64, t398: f64, t5674: f64, t1165: f64, t4282: f64, t5249: f64, t530: f64) -> (f64, f64, f64, f64) {
    let t23944 = t3237 * t1856;
    let t23946 = t1008 * t6200;
    let t23951 = t384 * t398 * t1095 * t5674 * t322;
    let t23959 = t4282 * t1165 * t530 * t5249;
    (t23944, t23946, t23951, t23959)
}
