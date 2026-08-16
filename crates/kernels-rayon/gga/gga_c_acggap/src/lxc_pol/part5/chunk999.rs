//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 999/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk999(t1181: f64, t3346: f64, t3361: f64, t535: f64, t3809: f64, t3382: f64, t4331: f64, t1324: f64, t3570: f64, t5082: f64, t952: f64, t4380: f64, t4389: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16724 = t3361 * t1181 * t535 * t3346;
    let t16728 = t3361 * t1181 * t535 * t3809;
    let t16730 = t3382 * t4331;
    let t16739 = t3570 * t1324;
    let t16745 = t952 * t5082;
    let t16755 = t4389 * t4380;
    (t16724, t16728, t16730, t16739, t16745, t16755)
}
