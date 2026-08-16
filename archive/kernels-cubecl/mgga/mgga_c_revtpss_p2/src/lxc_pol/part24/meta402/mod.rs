//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1337;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta402<F: Float>(t40196: F, t760: F, t10696: F, t73: F, t138: F, t785: F, t9302: F, t234: F, t39545: F, t685: F, t875: F, t2778: F, t39515: F, t39501: F, t871: F, t10115: F, t225: F, t10866: F, t232: F, t235: F, t239: F, t820: F, t2723: F, t2482: F, t2719: F, t596: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40198, t40231, t40270, t40294, t40314) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1337::<F>(t40196, t760, t10696, t73, t138, t785, t9302, t234, t39545, t685, t875, t2778, t39515);
        let (t40316, t40317, t40321, t40324, t40325, t40336) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338::<F>(t39501, t871, t10115, t225, t10866, t232, t235, t239, t820, t2723, t2482, t2719, t596);
    (t40198, t40231, t40270, t40294, t40314, t40316, t40317, t40321, t40324, t40325, t40336)
}
