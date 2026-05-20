//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta967 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta967<F: Float>(t73481: F, t73493: F, t73515: F, t74106: F, t48280: F, t48282: F, t48285: F, t74111: F, t48287: F, t47067: F, t47070: F, t47072: F, t47076: F, t48279: F, t48291: F, t48293: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t85918, t85919, t85920, t85921, t85922, t85923, t85924, t85925, t85926, t85927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3265::<F>(t73481, t73493, t73515, t74106, t48280, t48282, t48285, t74111, t48287, t47067, t47070, t47072, t47076, t48279, t48291, t48293);
    (t85918, t85919, t85920, t85921, t85922, t85923, t85924, t85925, t85926, t85927)
}
