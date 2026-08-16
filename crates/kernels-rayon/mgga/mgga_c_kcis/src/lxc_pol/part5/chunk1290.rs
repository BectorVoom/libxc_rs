//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1290/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1290(t21310: f64, t482: f64, t11536: f64, t7002: f64, t11539: f64, t1354: f64, t11388: f64, t5619: f64, t3918: f64, t7019: f64, t1578: f64, t5595: f64, t6114: f64) -> (f64, f64, f64, f64, f64) {
    let t21311 = t21310 * t482;
    let t21314 = t11536 * t7002;
    let t21315 = t11539 * t1354;
    let t21316 = t21314 * t21315;
    let t21319 = t11388 * t7002;
    let t21320 = t21319 * t5619;
    let t21323 = t3918 * t7019;
    let t21324 = t21323 * t1578;
    let t21327 = t5595 * t6114;
    (t21311, t21316, t21320, t21324, t21327)
}
