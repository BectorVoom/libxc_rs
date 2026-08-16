//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1011/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1011(t14909: f64, t14910: f64, t3952: f64, t3973: f64, t4402: f64, t1580: f64, t4384: f64, t4397: f64, t14875: f64, t14878: f64, t14886: f64, t14893: f64, t14898: f64, t14902: f64, t14906: f64, t1583: f64, t4381: f64, t4388: f64, t4393: f64, t4403: f64) -> f64 {
    let t14911 = t14909 * t14910;
    let t14912 = t3952 * t14911;
    let t14915 = t3973 * t4402;
    let t14916 = t1580 * t14915;
    let t14918 = t4397 * t4384;
    let t14920 = 0.16191709844559585492e0_f64 * t1580 * t14875 - 0.14392630972941853771e0_f64 * t14878 * t1583 - 0.71963154864709268853e-1_f64 * t4381 * t4388 - 0.95950873152945691806e-1_f64 * t4381 * t4393 + 0.2698618307426597582e-1_f64 * t14886 * t1583 - 0.53972366148531951639e-1_f64 * t4397 * t4403 - 0.2698618307426597582e-1_f64 * t1580 * t14893 - 0.2698618307426597582e-1_f64 * t1580 * t14898 - 0.53972366148531951639e-1_f64 * t1580 * t14902 + 0.35981577432354634428e-1_f64 * t1580 * t14906 - 0.35981577432354634427e-1_f64 * t1580 * t14912 - 0.17990788716177317214e-1_f64 * t14916 + 0.17990788716177317214e-1_f64 * t14918;
    t14920
}
