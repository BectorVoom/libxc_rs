//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1023/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1023(t1426: f64, t1579: f64, t2085: f64, t598: f64, t4959: f64, t7647: f64, t30148: f64, t5606: f64, t7585: f64, t7842: f64, t1181: f64, t23745: f64, t604: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t34089 = t598 * t1426 * t1579 * t2085;
    let t34091 = t7647 * t4959;
    let t34092 = 0.17149607247227894789e-2_f64 * t34091;
    let t34095 = t7585 * t7842 * t30148 * t5606;
    let t34099 = t7493 * t1181 * t604 * t23745;
    (t34089, t34092, t34095, t34099)
}
