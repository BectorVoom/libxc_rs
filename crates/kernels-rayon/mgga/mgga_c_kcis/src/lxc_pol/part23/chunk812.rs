//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 812/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk812(t15007: f64, t85: f64, t12274: f64, t2003: f64, t1396: f64, t531: f64, t1395: f64, t5780: f64, t6019: f64, t1498: f64, t1464: f64, t11783: f64, t2002: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15008 = t85 * t15007;
    let t15800 = t12274 * t2003;
    let t15802 = t1396 * t531;
    let t15803 = t1395 * t15802;
    let t15804 = t5780 * t15803;
    let t15808 = t6019 * sigma2;
    let t15809 = t15808 * t1498;
    let t15810 = t1464 * t15809;
    let t15812 = t11783 * t2002;
    (t15008, t15800, t15802, t15804, t15808, t15810, t15812)
}
