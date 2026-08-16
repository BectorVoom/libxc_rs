//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1082/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1082(t3233: f64, t7726: f64, t303: f64, t10463: f64, t1250: f64, t291: f64, t9985: f64, t2809: f64, t5329: f64, t1014: f64, t7735: f64, t1009: f64, t2909: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26778 = t7726 * t3233;
    let t26779 = t303 * t26778;
    let t26781 = t10463 * t1250;
    let t26782 = t291 * t9985;
    let t26783 = t26782 * t2809;
    let t26784 = t5329 * t26783;
    let t26787 = t1014 * t7735;
    let t26791 = t2909 * t1009;
    (t26778, t26779, t26781, t26783, t26784, t26787, t26791)
}
