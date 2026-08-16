//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1453/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1453(t3941: f64, t6534: f64, t7801: f64, t7769: f64, t84033: f64, t20173: f64, t33659: f64, t7056: f64, t7467: f64, t24462: f64, t1458: f64, t7263: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122837 = 27.0_f64 * t3941 * t7801 * t6534;
    let t122839 = 27.0_f64 * t84033 * t7769;
    let t122841 = 27.0_f64 * t20173 * t33659;
    let t122844 = 27.0_f64 * t3941 * t7056 * t7467;
    let t122846 = 0.135e2_f64 * t24462 * t7467;
    let t122917 = t7263 * t1458;
    (t122837, t122839, t122841, t122844, t122846, t122917)
}
