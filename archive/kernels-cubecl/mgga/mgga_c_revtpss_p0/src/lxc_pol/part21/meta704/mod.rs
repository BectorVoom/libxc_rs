//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2528;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta704<F: Float>(t3900: F, t9292: F, t1419: F, t9646: F, t9648: F, t10147: F, t1357: F, t689: F, t1362: F, t1363: F, t39497: F, t1358: F, t588: F, t9647: F, t4086: F, t1399: F, t22: F, t555: F, t9890: F, t10040: F, t2435: F, t10039: F, t2439: F, t2777: F, t4003: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46369, t46378, t46381, t46385, t46388) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2528::<F>(t3900, t9292, t1419, t9646, t9648, t10147, t1357, t689, t1362, t1363, t39497, t1358, t588, t9647);
        let (t46389, t46392, t46394, t46398, t46401, t46403) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2529::<F>(t4086, t9646, t1399, t22, t555, t9890, t10040, t2435, t10039, t2439, t2777, t4003);
    (t46369, t46378, t46381, t46385, t46388, t46389, t46392, t46394, t46398, t46401, t46403)
}
