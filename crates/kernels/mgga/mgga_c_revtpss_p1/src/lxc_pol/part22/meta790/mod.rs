//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2881;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta790<F: Float>(t39552: F, t562: F, t560: F, t9655: F, t225: F, t3896: F, t39515: F, t3900: F, t9292: F, t1419: F, t9646: F, t9648: F, t1362: F, t1363: F, t39497: F, t1358: F, t588: F, t9647: F, t4086: F, t1399: F, t22: F, t555: F, t10040: F, t2435: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46359, t46362, t46368, t46369, t46378) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2881::<F>(t39552, t562, t560, t9655, t225, t3896, t39515, t3900, t9292, t1419, t9646, t9648);
        let (t46385, t46388, t46389, t46392, t46398) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2882::<F>(t1362, t1363, t39497, t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t10040, t2435);
    (t46359, t46362, t46368, t46369, t46378, t46385, t46388, t46389, t46392, t46398)
}
