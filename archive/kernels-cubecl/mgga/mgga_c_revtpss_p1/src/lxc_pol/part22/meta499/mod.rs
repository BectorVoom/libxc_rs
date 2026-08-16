//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta499<F: Float>(t1678: F, t3151: F, t3304: F, t3302: F, t4893: F, t15609: F, t15604: F, t1089: F, t1668: F, t3259: F, t15780: F, t4983: F) -> (F, F, F, F, F, F, F) {
        let (t16426, t16427, t16432, t16433, t16436, t16440, t16443) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2232::<F>(t1678, t3151, t3304, t3302, t4893, t15609, t15604, t1089, t1668, t3259, t15780, t4983);
    (t16426, t16427, t16432, t16433, t16436, t16440, t16443)
}
