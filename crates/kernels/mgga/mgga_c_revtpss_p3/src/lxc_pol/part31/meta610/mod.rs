//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2051;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta610<F: Float>(t28002: F, t686: F, t72: F, t25895: F, t5722: F, t94748: F, t98067: F, t27968: F, t3920: F, t1445: F, t27985: F, t689: F, t5775: F, t7242: F, t25898: F, t98040: F, t25901: F, t25878: F, t27989: F, t94921: F, t94802: F, t25899: F, t98303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98356, t98358, t98360, t98368, t98372, t98376) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2051::<F>(t28002, t686, t72, t25895, t5722, t94748, t98067, t27968, t3920, t1445, t27985, t689);
        let (t98379, t98380, t98382, t98384, t98387, t98390, t98399) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2052::<F>(t5775, t689, t7242, t25898, t98040, t25901, t25878, t98356, t27989, t94921, t94802, t25899, t98303);
    (t98358, t98360, t98368, t98372, t98376, t98379, t98380, t98382, t98384, t98387, t98390, t98399)
}
