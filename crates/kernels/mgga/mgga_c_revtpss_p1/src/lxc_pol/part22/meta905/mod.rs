//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta905 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3103;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta905<F: Float>(t15752: F, t16049: F, t16087: F, t53884: F, t15988: F, t3241: F, t1011: F, t15158: F, t15987: F, t15994: F, t43537: F, t53668: F, t11933: F, t16035: F, t11774: F, t127: F, t15585: F, t4872: F, t16226: F, t16229: F, t53405: F, t3230: F, t4857: F, t11817: F, t4858: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54261, t54289, t54303, t54306, t54314, t54316) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3103::<F>(t15752, t16049, t16087, t53884, t15988, t3241, t1011, t15158, t15987, t15994, t43537, t53668);
        let (t54324, t54341, t54348, t54384, t54387) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104::<F>(t11933, t16035, t11774, t127, t15585, t4872, t16226, t16229, t53405, t3230, t4857, t11817, t4858);
    (t54261, t54289, t54303, t54306, t54314, t54316, t54324, t54341, t54348, t54384, t54387)
}
