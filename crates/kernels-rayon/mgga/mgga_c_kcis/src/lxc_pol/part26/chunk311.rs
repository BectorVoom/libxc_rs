//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 311/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk311(t1615: f64, t1616: f64, t1390: f64, t1399: f64, t1461: f64, t1492: f64, t1500: f64, t1588: f64, t1592: f64, t626: f64, t632: f64) -> (f64, f64, f64, f64, f64) {
    let t1617 = t1615 * t1616;
    let t1620 = 0.11607361111111111111e-2_f64 * t1390;
    let t1625 = t1588 * t626 - 0.66725e-1_f64 * t1592 * t1617 + t1620 + 0.11607361111111111111e-2_f64 * t1399 + 0.17411041666666666666e-2_f64 * t1461 - 0.17411041666666666666e-2_f64 * t1492 + 0.11607361111111111111e-2_f64 * t1500;
    let t1627 = t632 * t632;
    let t1628 = 1.0_f64 / t1627;
    (t1617, t1620, t1625, t1627, t1628)
}
