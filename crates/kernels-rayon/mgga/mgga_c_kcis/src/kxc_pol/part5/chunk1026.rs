//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1026/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1026(t10513: f64, t284: f64, t41: f64, t9545: f64, t3436: f64, t9588: f64, t1094: f64, t5163: f64, t1780: f64, t245: f64, t3393: f64, t5155: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14832 = t10513 * t284;
    let t14838 = t41 * t9545;
    let t14849 = t9588 * t3436;
    let t14874 = t5163 * t1094;
    let t14875 = t14874 * sigma0;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    (t14832, t14838, t14849, t14875, t14907, t14913)
}
