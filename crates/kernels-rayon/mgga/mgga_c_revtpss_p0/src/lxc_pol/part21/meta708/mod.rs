//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2536;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta708(t1438: f64, t40317: f64, t10065: f64, t10069: f64, t2782: f64, t4086: f64, t46469: f64, t543: f64, t10084: f64, t1398: f64, t4066: f64, t10079: f64, t1419: f64, t3923: f64, t4089: f64, t40921: f64, t10073: f64, t4003: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46526, t46536, t46540, t46542, t46561, t46563) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2536(t1438, t40317, t10065, t10069, t2782, t4086, t46469, t543, t10084, t1398, t4066, t10079);
        let (t46565, t46568, t46570, t46572, t46583) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2537(t1419, t3923, t2782, t4086, t543, t4089, t40921, t10073, t10079, t4003, t46469, t5744);
    (t46526, t46536, t46540, t46542, t46561, t46563, t46565, t46568, t46570, t46572, t46583)
}
