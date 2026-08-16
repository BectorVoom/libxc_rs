//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta847 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2985;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta847<F: Float>(t14141: F, t14143: F, t4056: F, t676: F, t14066: F, t1432: F, t686: F, t72: F, t14188: F, t2439: F, t2777: F, t10073: F, t14129: F, t14159: F, t3964: F, t9285: F, t213: F, t225: F, t46475: F, t10019: F, t14114: F, t14145: F, t2482: F, t4114: F, t5658: F) -> (F, F, F, F, F, F, F, F) {
        let (t49403, t49407, t49426, t49429) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2985::<F>(t14141, t14143, t4056, t676, t14066, t1432, t686, t72, t14188, t2439, t2777, t10073, t14129);
        let (t49432, t49439, t49446, t49450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986::<F>(t14159, t3964, t9285, t213, t225, t46475, t10019, t14114, t14145, t2482, t4114, t5658);
    (t49403, t49407, t49426, t49429, t49432, t49439, t49446, t49450)
}
