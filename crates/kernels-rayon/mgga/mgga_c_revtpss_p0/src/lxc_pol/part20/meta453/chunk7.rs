//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1736/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736(t46929: f64, t808: f64, t9935: f64, t9845: f64, t9930: f64, t9769: f64, t2713: f64, t3964: f64, t9703: f64, t4086: f64, t9801: f64, t9846: f64) -> (f64, f64, f64, f64, f64) {
    let t46931 = t46929 * t808 * t9935;
    let t46934 = t9845 * t808 * t9930;
    let t46941 = t9845 * t808 * t9769;
    let t46944 = t3964 * t2713 * t9703;
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    (t46931, t46934, t46941, t46944, t46947)
}
