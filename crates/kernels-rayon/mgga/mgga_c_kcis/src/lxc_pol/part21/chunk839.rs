//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 839/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk839(t11041: f64, t1251: f64, t110: f64, t992: f64, t1254: f64, t3500: f64, t3525: f64, t25: f64, t2887: f64, t3509: f64, t3530: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11042 = t1251 * t11041;
    let t11061 = t110 * t992;
    let t11062 = t11061 * t1254;
    let t11063 = t1251 * t11062;
    let t11065 = t3500 * t3525;
    let t11066 = t1251 * t11065;
    let t11068 = t25 * t2887;
    let t11069 = t11068 * t3509;
    let t11070 = t1251 * t11069;
    let t11072 = t993 * t3530;
    (t11042, t11061, t11063, t11066, t11068, t11070, t11072)
}
