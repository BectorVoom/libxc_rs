//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2556;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta718(t1399: f64, t3960: f64, t9816: f64, t9818: f64, t2735: f64, t5744: f64, t808: f64, t9935: f64, t9845: f64, t9930: f64, t9769: f64, t2713: f64, t3964: f64, t9703: f64, t4086: f64, t9801: f64, t9846: f64, t9744: f64, t9966: f64, t3855: f64, t3860: f64, t1320: f64, t9545: f64, t3857: f64, t40082: f64, t512: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46922, t46931, t46934, t46941, t46944) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2556(t1399, t3960, t9816, t9818, t2735, t5744, t808, t9935, t9845, t9930, t9769, t2713, t3964, t9703);
        let (t46946, t46947, t46949, t46960, t46963, t46967, t46970) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557(t4086, t9801, t9846, t9744, t9966, t3855, t3860, t1320, t9545, t3857, t40082, t512, t520);
    (t46922, t46931, t46934, t46941, t46944, t46946, t46947, t46949, t46960, t46963, t46967, t46970)
}
