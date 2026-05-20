//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2372;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta648<F: Float>(t138: F, t785: F, t9302: F, t2786: F, t234: F, t39545: F, t685: F, t875: F, t2778: F, t39515: F, t39501: F, t871: F, t10115: F, t225: F, t880: F, t10866: F, t232: F, t235: F, t2723: F, t2482: F, t2719: F, t596: F, t10868: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40270, t40271, t40294, t40314, t40316) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2372::<F>(t138, t785, t9302, t2786, t234, t39545, t685, t875, t2778, t39515, t39501, t871);
        let (t40317, t40318, t40321, t40322, t40325, t40336, t40348) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2373::<F>(t10115, t225, t880, t10866, t232, t235, t2723, t2482, t2719, t596, t10868, t820, t843);
    (t40270, t40271, t40294, t40314, t40316, t40317, t40318, t40321, t40322, t40325, t40336, t40348)
}
