//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta833 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2956;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta833<F: Float>(t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t46917: F, t5701: F, t14005: F, t46740: F, t5697: F, t1872: F, t4057: F, t9816: F, t9818: F, t13824: F, t221: F, t3978: F, t46716: F, t13923: F, t3930: F, t14036: F, t9976: F, t46694: F, t5686: F, t14030: F, t9744: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48600, t48603, t48614, t48637, t48645, t48655) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2956::<F>(t13955, t46946, t13775, t808, t9845, t46917, t5701, t14005, t46740, t5697, t1872, t4057, t9816, t9818);
        let (t48664, t48666, t48668, t48685, t48687) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2957::<F>(t13824, t221, t3978, t46716, t13923, t3930, t14036, t9976, t46694, t5686, t14030, t9744);
    (t48600, t48603, t48614, t48637, t48645, t48655, t48664, t48666, t48668, t48685, t48687)
}
