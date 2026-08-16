//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 586/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk586(t645: f64, t4971: f64, t4972: f64, t1755: f64, t4803: f64, t1751: f64, t1758: f64, t340: f64, t4962: f64, t639: f64, t642: f64, t655: f64, t1765: f64, t1769: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t646 = t645 < -0.66725e-1_f64;
    let t4973 = t4971 * t4972;
    let t4977 = t1755 * t4803;
    let t4982 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t4962 * t642 - 20.0_f64 / 27.0_f64 * t340 * t1751 * t1758 + 40.0_f64 / 81.0_f64 * t340 * t639 * t4973 - 10.0_f64 / 27.0_f64 * t340 * t639 * t4977);
    let t4983 = t4982 * sigma2;
    let t4984 = t4983 * t655;
    let t4987 = t1765 * t1769;
    (t4973, t4977, t4983, t4984, t4987)
}
