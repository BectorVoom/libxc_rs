//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 949/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk949(t1495: f64, t21971: f64, t1395: f64, t1464: f64, t16752: f64, t2002: f64, t11369: f64, t1319: f64, t6937: f64, t11374: f64, t1419: f64, t21125: f64, t5425: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21972 = t1495 * t21971;
    let t21973 = t1395 * t21972;
    let t21974 = t1464 * t21973;
    let t21976 = t16752 * t2002;
    let t21977 = t1464 * t21976;
    let t21983 = t11369 * t6937 * t1319;
    let t21987 = t11374 * t6937 * t1419;
    let t21990 = t5425 * t21125;
    (t21972, t21974, t21977, t21983, t21987, t21990)
}
