//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1056/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1056(t1499: f64, t5155: f64, t1508: f64, t4885: f64, t496: f64, t4803: f64, t513: f64, t5142: f64, t1527: f64, t1516: f64, t491: f64, t1599: f64, t1601: f64, t490: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16498 = t5155 * t1499;
    let t16500 = t5155 * t1508;
    let t16502 = t496 * t4885;
    let t16506 = t4803 * t513;
    let t16508 = t5142 * t513;
    let t16510 = t1527 * t1527;
    let t16513 = 6.0_f64 * t1516 * t16510 * t491;
    let t16517 = 0.64327917994770140268e2_f64 * t1599 * t4993 * t1601 * t490;
    (t16498, t16500, t16502, t16506, t16508, t16510, t16513, t16517)
}
