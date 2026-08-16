//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 717/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk717(t645: f64, t10975: f64, t11136: f64, t67: f64, t1848: f64, t641: f64, t916: f64, t1757: f64, t4972: f64, t10522: f64, t1755: f64, t10436: f64, t1751: f64, t1758: f64, t340: f64, t4962: f64, t4973: f64, t4977: f64, t6141: f64, t639: f64, t642: f64, t7196: f64) -> (f64, f64, f64, f64) {
    let t646 = t645 < -0.66725e-1_f64;
    let t11138 = t67 * (t10975 + t11136);
    let t11153 = 1.0_f64 / t641 / t916 / t1848;
    let t11154 = t4972 * t1757;
    let t11155 = t11153 * t11154;
    let t11162 = t1755 * t10522;
    let t11167 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t11138 * t642 - 10.0_f64 / 9.0_f64 * t340 * t4962 * t1758 + 40.0_f64 / 27.0_f64 * t340 * t1751 * t4973 - 10.0_f64 / 9.0_f64 * t340 * t1751 * t4977 - 280.0_f64 / 243.0_f64 * t340 * t639 * t11155 + 40.0_f64 / 27.0_f64 * t6141 * t7196 * t10436 - 10.0_f64 / 27.0_f64 * t340 * t639 * t11162);
    (t11154, t11155, t11162, t11167)
}
