//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta732 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2578;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta732<F: Float>(t10061: F, t10069: F, t2782: F, t4086: F, t46407: F, t543: F, t4003: F, t46565: F, t5744: F, t10073: F, t10111: F, t1428: F, t588: F, t4066: F, t786: F, t4104: F, t4100: F, t46433: F, t10022: F, t2453: F, t281: F, t46507: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47403, t47407, t47411, t47413, t47417) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2578::<F>(t10061, t10069, t2782, t4086, t46407, t543, t4003, t46565, t5744, t10073, t10111, t1428, t588);
        let (t47423, t47424, t47427, t47432) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2579::<F>(t4066, t4086, t786, t4104, t2782, t4100, t46433, t10022, t2453, t281, t4003, t46507);
    (t47403, t47407, t47411, t47413, t47417, t47423, t47424, t47427, t47432)
}
