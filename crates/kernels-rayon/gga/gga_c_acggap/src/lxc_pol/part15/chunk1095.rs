//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1095/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1095(t1988: f64, t9681: f64, t1841: f64, t7685: f64, t1426: f64, t429: f64, t598: f64, t9536: f64, t137: f64, t5506: f64, t368: f64, t1980: f64, t38889: f64, t7476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38914 = t1988 * t9681;
    let t38916 = t7685 * t1841;
    let t38920 = t598 * t1426 * t429 * t9536;
    let t38922 = t137 * t5506;
    let t38925 = t598 * t1426 * t368 * t38922;
    let t38929 = t1980 * t7476 * t38889;
    (t38914, t38916, t38920, t38922, t38925, t38929)
}
