//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 894/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk894(t1876: f64, t1877: f64, t28312: f64, t22937: f64, t2487: f64, t7028: f64, t7029: f64, t8522: f64, t2364: f64, t4604: f64, t4609: f64, t8518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28911 = t1876 * t1877 * t28312;
    let t28914 = t22937 * t2487;
    let t28915 = t7028 * t28914;
    let t28918 = t7029 * t8522;
    let t28919 = t7028 * t28918;
    let t28925 = t4604 * t2364 * t8522;
    let t28929 = t4609 * t8518 * t2487;
    (t28911, t28914, t28915, t28918, t28919, t28925, t28929)
}
