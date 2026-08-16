//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2605;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta740(t13726: f64, t9303: f64, t13725: f64, t1445: f64, t2439: f64, t14082: f64, t3920: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64, t10119: f64, t14114: f64, t10115: f64, t1900: f64, t14189: f64, t22: f64, t46389: f64, t543: f64, t5735: f64, t1432: f64, t5763: f64, t9288: f64, t1892: f64, t3923: f64, t2782: f64, t4003: f64, t5744: f64, t10069: f64, t14124: f64, t14129: f64, t14231: f64, t14255: f64, t4057: f64, t46443: f64, t46448: f64, t46452: f64, t46454: f64, t46458: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47938, t47942, t47945, t47948, t47952) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604(t13726, t9303, t13725, t1445, t2439, t14082, t3920, t14078, t2470, t3915, t13735, t2435);
        let (t47953, t47957, t47961, t47964, t47967, t47971) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2605(t47952, t10119, t14114, t10115, t1900, t14189, t2435, t22, t46389, t543, t5735, t1432, t5763, t9288);
        let (t47973, t47992) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606(t1892, t3923, t2782, t4003, t5744, t10069, t14124, t14129, t14231, t14255, t4057, t46443, t46448, t46452, t46454, t46458, t47971, t820);
    (t47938, t47942, t47945, t47948, t47953, t47957, t47961, t47964, t47967, t47973, t47992)
}
