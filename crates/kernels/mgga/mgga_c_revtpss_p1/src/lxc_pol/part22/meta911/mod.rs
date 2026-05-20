//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta911 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta911<F: Float>(t11223: F, t16088: F, t380: F, t1041: F, t16185: F, t3172: F, t1062: F, t42261: F, t11710: F, t15974: F, t4899: F, t11866: F, t15794: F, t11671: F, t15925: F, t15752: F, t15917: F, t127: F, t15700: F, t15702: F, t4801: F, t1063: F, t11986: F, t247: F, t4583: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54857, t54869, t54899, t54907, t54914) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115::<F>(t11223, t16088, t380, t1041, t16185, t3172, t1062, t42261, t11710, t15974, t4899, t11866, t15794);
        let (t54916, t54919, t54925, t54943) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3116::<F>(t11671, t15925, t15752, t15917, t127, t15700, t15702, t4801, t1063, t11986, t247, t4583);
    (t54857, t54869, t54899, t54907, t54914, t54916, t54919, t54925, t54943)
}
