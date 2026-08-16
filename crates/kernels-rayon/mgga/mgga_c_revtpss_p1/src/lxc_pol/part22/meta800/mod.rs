//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta800 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta800(t3940: f64, t46917: f64, t3829: f64, t4003: f64, t2735: f64, t5744: f64, t808: f64, t9935: f64, t9845: f64, t9930: f64, t9769: f64, t2713: f64, t3964: f64, t9703: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46918, t46924, t46929, t46931, t46934, t46941, t46944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2901(t3940, t46917, t3829, t4003, t2735, t5744, t808, t9935, t9845, t9930, t9769, t2713, t3964, t9703);
    (t46918, t46924, t46929, t46931, t46934, t46941, t46944)
}
