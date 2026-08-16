//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2578;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta732(t10061: f64, t10069: f64, t2782: f64, t4086: f64, t46407: f64, t543: f64, t4003: f64, t46565: f64, t5744: f64, t10073: f64, t10111: f64, t1428: f64, t588: f64, t4066: f64, t786: f64, t4104: f64, t4100: f64, t46433: f64, t10022: f64, t2453: f64, t281: f64, t46507: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47403, t47407, t47411, t47413, t47417) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2578(t10061, t10069, t2782, t4086, t46407, t543, t4003, t46565, t5744, t10073, t10111, t1428, t588);
        let (t47423, t47424, t47427, t47432) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2579(t4066, t4086, t786, t4104, t2782, t4100, t46433, t10022, t2453, t281, t4003, t46507);
    (t47403, t47407, t47411, t47413, t47417, t47423, t47424, t47427, t47432)
}
