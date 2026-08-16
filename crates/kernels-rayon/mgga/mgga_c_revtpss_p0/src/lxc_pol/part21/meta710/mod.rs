//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2540;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta710(t550: f64, t9898: f64, t2661: f64, t46609: f64, t9994: f64, t3992: f64, t543: f64, t9890: f64, t3995: f64, t40488: f64, t3989: f64, t9944: f64, t549: f64, t240: f64, t72: f64, t4014: f64, t9779: f64, t221: f64, t3978: f64, t3979: f64, t9628: f64, t1408: f64, t2237: f64, t2482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46610, t46613, t46618, t46620, t46622) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2540(t550, t9898, t2661, t46609, t9994, t3992, t543, t9890, t3995, t40488, t3989, t9944);
        let (t46627, t46633, t46641, t46644) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2541(t549, t240, t72, t4014, t9779, t221, t3978, t3979, t9628, t1408, t2237, t2482);
    (t46610, t46613, t46618, t46620, t46622, t46627, t46633, t46641, t46644)
}
