//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2444/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2444(t1390: f64, t1399: f64, t46856: f64, t685: f64, t3952: f64, t9784: f64, t281: f64, t39644: f64, t40650: f64, t547: f64, t550: f64, t40688: f64) -> (f64, f64, f64, f64) {
    let t46859 = t46856 * t1390 * t685 * t1399;
    let t46879 = t9784 * t3952;
    let t46885 = 0.47607864835161149081e-7_f64 * t39644 * t547 * t40650 * t550 * t281;
    let t46888 = t40688 * t547;
    (t46859, t46879, t46885, t46888)
}
