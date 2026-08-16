//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta841<F: Float>(t13910: F, t808: F, t9736: F, t14026: F, t9744: F, t13821: F, t13999: F, t13716: F, t1413: F, t547: F, t807: F, t550: F, t9794: F, t14224: F, t9793: F, t13928: F, t9962: F, t13800: F, t46670: F, t3964: F, t5617: F, t9732: F, t136: F, t216: F, t9747: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49056, t49058, t49062, t49066, t49068) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972::<F>(t13910, t808, t9736, t14026, t9744, t13821, t13999, t13716, t1413, t547, t807, t550, t9794);
        let (t49070, t49085, t49087, t49090, t49093) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2973::<F>(t14224, t49068, t9793, t13928, t9962, t13800, t46670, t3964, t5617, t9732, t136, t216, t9747);
    (t49056, t49058, t49062, t49066, t49068, t49070, t49085, t49087, t49090, t49093)
}
