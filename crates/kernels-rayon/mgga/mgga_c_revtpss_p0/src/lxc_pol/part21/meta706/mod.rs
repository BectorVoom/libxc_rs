//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2532;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta706(t46456: f64, t786: f64, t10026: f64, t1398: f64, t268: f64, t4101: f64, t543: f64, t793: f64, t10073: f64, t10084: f64, t555: f64, t9898: f64, t14192: f64, t2782: f64, t9994: f64, t544: f64, t9989: f64, t4003: f64, t215: f64, t4056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46458, t46463, t46465, t46469) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2532(t46456, t786, t10026, t1398, t268, t4101, t543, t793, t10073, t10084, t555, t9898);
        let (t46472, t46475, t46478, t46490) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2533(t14192, t2782, t46469, t9994, t544, t9989, t4003, t215, t268, t4056, t4101, t543);
    (t46458, t46463, t46465, t46469, t46472, t46475, t46478, t46490)
}
