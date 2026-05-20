//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2912;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta810<F: Float>(t245: F, t47247: F, t2713: F, t3964: F, t9714: F, t3951: F, t9732: F, t136: F, t4010: F, t220: F, t9905: F, t9976: F, t3926: F, t9909: F, t9775: F, t9981: F, t1389: F, t40604: F, t3961: F, t9741: F, t10111: F, t22: F, t4092: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47248, t47259, t47262, t47273, t47274, t47298) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2912::<F>(t245, t47247, t2713, t3964, t9714, t3951, t9732, t136, t4010, t220, t9905, t9976);
        let (t47304, t47320, t47337, t47338, t47348) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913::<F>(t3926, t9909, t9775, t9981, t1389, t3964, t40604, t3961, t9741, t10111, t22, t4092);
    (t47248, t47259, t47262, t47273, t47274, t47298, t47304, t47320, t47337, t47338, t47348)
}
