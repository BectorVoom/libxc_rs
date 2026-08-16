//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 915/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk915(t14046: f64, t7336: f64, t7643: f64, t1973: f64, t7630: f64, t1985: f64, t30231: f64, t1967: f64, t7792: f64, t7637: f64, t7796: f64, t1980: f64, t1982: f64, t1992: f64, t5: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30984 = t14046 * t7336;
    let t30985 = t30984 * t7643;
    let t30987 = t7630 * t1973;
    let t30989 = t30231 * t1985;
    let t30990 = 0.28582678745379824648e-2_f64 * t30989;
    let t30991 = t1967 * t7792;
    let t30993 = t7637 * t7796;
    let t30998 = t1980 * t1982 * t5 * t965 * t1992;
    (t30984, t30985, t30987, t30990, t30991, t30993, t30998)
}
