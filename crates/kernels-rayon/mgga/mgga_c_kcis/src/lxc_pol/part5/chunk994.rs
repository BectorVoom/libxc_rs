//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 994/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk994(t1370: f64, t4455: f64, t1607: f64, t3978: f64, t1606: f64, t4354: f64, t597: f64, t592: f64, t11407: f64, t11481: f64, t1562: f64, t4357: f64, t600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12605 = t1370 * t4455;
    let t12617 = t3978 * t1607;
    let t12650 = t1606 * t1606;
    let t12651 = 1.0_f64 / t12650;
    let t12688 = 1.0_f64 / t4354 / t597;
    let t12689 = t592 * t12688;
    let t12717 = 0.16068111111111111111e1_f64 * t11407;
    let t12718 = 0.46308888888888888888e0_f64 * t11481;
    let t12729 = 1.0_f64 / t4354 / t1562;
    let t12730 = t592 * t12729;
    let t12732 = 1.0_f64 / t4357 / t600;
    (t12605, t12617, t12651, t12689, t12717, t12718, t12730, t12732)
}
