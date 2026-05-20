//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1040 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1040<F: Float>(t12227: F, t3385: F, t6474: F, t16942: F, t1733: F, t3384: F, t12248: F, t3427: F, t20651: F, t44017: F, t6471: F, t20644: F, t3433: F, t68738: F, t68742: F, t68744: F, t68746: F, t68748: F, t68751: F) -> (F, F, F, F, F, F, F, F) {
        let (t68754, t68757, t68760, t68763, t68766, t68769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632::<F>(t12227, t3385, t6474, t16942, t1733, t3384, t12248, t3427, t20651, t44017, t6471, t20644);
        let (t68772, t68773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3633::<F>(t20644, t3427, t3433, t68738, t68742, t68744, t68746, t68748, t68751, t68754, t68757, t68760, t68763, t68766, t68769);
    (t68754, t68757, t68760, t68763, t68766, t68769, t68772, t68773)
}
