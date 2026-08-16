//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1054/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1054(t4803: f64, t513: f64, t5142: f64, t1527: f64, t1516: f64, t491: f64, t1599: f64, t1601: f64, t490: f64, t4993: f64, t1597: f64, t1517: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16506 = t4803 * t513;
    let t16508 = t5142 * t513;
    let t16510 = t1527 * t1527;
    let t16513 = 6.0_f64 * t1516 * t16510 * t491;
    let t16517 = 0.64327917994770140268e2_f64 * t1599 * t4993 * t1601 * t490;
    let t16518 = t1597 * t1597;
    let t16521 = t1517 * t1517;
    (t16506, t16508, t16510, t16513, t16517, t16518, t16521)
}
