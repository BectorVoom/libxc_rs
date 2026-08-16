//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta470<F: Float>(t18353: F, t2689: F, t18348: F, t2710: F, t2713: F, t18562: F, t2626: F, t2609: F, t5944: F, t10815: F, t5980: F, t40398: F, t6024: F, t10716: F, t18423: F, t18415: F, t9775: F, t18410: F, t10995: F, t18804: F, t2470: F, t18725: F, t2798: F, t10069: F, t18738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t62129, t62251, t62276, t62300, t62399, t62401) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448::<F>(t18353, t2689, t18348, t2710, t2713, t18562, t2626, t2609, t5944, t10815, t5980, t40398, t6024);
        let (t62431, t62443, t62445, t62528, t62633, t62649) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1449::<F>(t10716, t18423, t18415, t9775, t18410, t10995, t18804, t2470, t18725, t2798, t10069, t18738);
    (t62129, t62251, t62276, t62300, t62399, t62401, t62431, t62443, t62445, t62528, t62633, t62649)
}
