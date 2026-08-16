//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta716(t2735: f64, t9792: f64, t1413: f64, t46826: f64, t1376: f64, t40769: f64, t3989: f64, t9986: f64, t10001: f64, t221: f64, t4019: f64, t9912: f64, t10111: f64, t1386: f64, t9720: f64, t1390: f64, t1399: f64, t685: f64, t9970: f64, t9976: f64, t3930: f64, t9893: f64, t3957: f64, t9700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46835, t46837, t46840, t46846, t46853) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552(t2735, t9792, t1413, t46826, t1376, t40769, t3989, t9986, t10001, t221, t4019, t9912);
        let (t46856, t46859, t46861, t46863, t46865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2553(t10111, t1386, t9720, t1390, t1399, t685, t9970, t9976, t3930, t9893, t3957, t9700);
    (t46835, t46837, t46840, t46846, t46853, t46856, t46859, t46861, t46863, t46865)
}
