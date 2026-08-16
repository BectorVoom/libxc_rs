//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 873/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk873(t1181: f64, t30209: f64, t3655: f64, t604: f64, t3044: f64, t599: f64, t3809: f64, t7493: f64, t3759: f64, t7426: f64, t3073: f64, t7646: f64) -> (f64, f64, f64, f64, f64) {
    let t30334 = t30209 * t1181 * t604 * t3655;
    let t30339 = t30209 * t1181 * t599 * t3044;
    let t30340 = 0.64311027177104605458e-3_f64 * t30339;
    let t30343 = t7493 * t1181 * t599 * t3809;
    let t30347 = t7426 * t1181 * t604 * t3759;
    let t30364 = t3073 * t7646;
    (t30334, t30340, t30343, t30347, t30364)
}
