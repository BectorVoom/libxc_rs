//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 783/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk783(t11913: f64, t4166: f64, t1401: f64, t4036: f64, t3754: f64, t89: f64, t4034: f64, t516: f64, t1445: f64, t4024: f64, t4028: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11914 = t11913 * t4166;
    let t11918 = t1401 * t4036;
    let t11920 = t89 * t3754;
    let t11939 = 1.0_f64 / t4034 / t516;
    let t11947 = t1445 * t4024;
    let t11949 = t532 * t4028;
    (t11914, t11918, t11920, t11939, t11947, t11949)
}
