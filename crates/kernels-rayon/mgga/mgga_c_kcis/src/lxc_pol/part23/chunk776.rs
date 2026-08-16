//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 776/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk776(t3946: f64, t481: f64, t1311: f64, t3860: f64, t11407: f64, t1315: f64, t3853: f64, t3898: f64, t3897: f64, t465: f64, t455: f64, t11481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11539 = 1.0_f64 / t3946 / t481;
    let t11543 = t1311 * t3860;
    let t11557 = 0.55403703703703703703e-1_f64 * t11407;
    let t11571 = t3853 * t1315;
    let t11576 = t1311 * t3898;
    let t11580 = 1.0_f64 / t3897 / t465;
    let t11581 = t455 * t11580;
    let t11608 = 0.93011851851851851854e0_f64 * t11407;
    let t11609 = 0.36514074074074074075e0_f64 * t11481;
    (t11539, t11543, t11557, t11571, t11576, t11581, t11608, t11609)
}
