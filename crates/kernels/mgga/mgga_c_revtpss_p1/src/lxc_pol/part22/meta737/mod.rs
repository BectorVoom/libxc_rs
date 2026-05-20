//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2797;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta737<F: Float>(t268: F, t40689: F, t2665: F, t10868: F, t240: F, t10722: F, t2656: F, t2237: F, t2482: F, t849: F, t2677: F, t234: F, t9801: F, t10887: F, t136: F, t2475: F, t220: F, t2668: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40690, t40691, t40693, t40707, t40710, t40711, t40721) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2797::<F>(t268, t40689, t2665, t10868, t240, t10722, t2656, t2237, t2482, t849, t2677, t234, t9801);
        let (t40722, t40724, t40725, t40731) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2798::<F>(t10887, t40721, t136, t2475, t220, t2482, t2668, t823);
    (t40690, t40691, t40693, t40707, t40710, t40711, t40721, t40722, t40724, t40725, t40731)
}
