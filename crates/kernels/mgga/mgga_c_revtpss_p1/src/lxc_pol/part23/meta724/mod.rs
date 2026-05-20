//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta724 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta724<F: Float>(t13951: F, t2713: F, t3964: F, t1413: F, t46835: F, t48698: F, t1873: F, t46651: F, t13910: F, t808: F, t9736: F, t550: F, t9794: F, t14224: F, t9793: F, t13800: F, t46670: F, t5617: F, t9732: F, t136: F, t216: F, t9747: F, t14230: F, t46802: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49008, t49012, t49030, t49057, t49068) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488::<F>(t13951, t2713, t3964, t1413, t46835, t48698, t1873, t46651, t13910, t808, t9736, t550, t9794);
        let (t49071, t49087, t49090, t49093, t49103) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2489::<F>(t14224, t49068, t9793, t13800, t46670, t3964, t5617, t9732, t136, t216, t9747, t14230, t46802);
    (t49008, t49012, t49030, t49057, t49071, t49087, t49090, t49093, t49103)
}
