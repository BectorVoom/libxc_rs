//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2118;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta453<F: Float>(t15193: F, t904: F, t128: F, t4628: F, t698: F, t930: F, t141: F, t15127: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11304: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15194, t15195) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2118::<F>(t15193, t904, t128);
        let (t15197, t15198, t15199, t15200, t15209, t15210, t15211, t15220) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2119::<F>(t4628, t698, t15193, t930, t141, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11304, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15194, t15195, t15197, t15198, t15199, t15200, t15209, t15210, t15211, t15220)
}
