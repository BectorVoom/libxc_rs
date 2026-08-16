//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1733/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1733(t3930: f64, t9893: f64, t3957: f64, t9700: f64, t1413: f64, t547: f64, t807: f64, t9628: f64, t3952: f64, t9784: f64, t281: f64, t39644: f64, t40650: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t46863 = t3930 * t9893;
    let t46865 = t3957 * t9700;
    let t46877 = t807 * t547 * t1413 * t9628;
    let t46879 = t9784 * t3952;
    let t46885 = 0.47607864835161149081e-7_f64 * t39644 * t547 * t40650 * t550 * t281;
    (t46863, t46865, t46877, t46879, t46885)
}
