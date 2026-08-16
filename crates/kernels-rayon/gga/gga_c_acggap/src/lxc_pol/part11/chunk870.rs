//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 870/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk870(t30248: f64, t425: f64, t1020: f64, t7614: f64, t1029: f64, t7605: f64, t7478: f64, t7637: f64, t1160: f64, t7584: f64, t1992: f64, t4210: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30249 = t30248 * t425;
    let t30251 = t7614 * t1020;
    let t30253 = t7605 * t1029;
    let t30260 = t7637 * t7478;
    let t30262 = t1160 * t7584;
    let t30265 = t30262 * t7842 * t1992 * t4210;
    (t30249, t30251, t30253, t30260, t30262, t30265)
}
