//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2550;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta758<F: Float>(t54122: F, t1011: F, t3252: F, t4574: F, t697: F, t11263: F, t4879: F, t43537: F, t53668: F, t11817: F, t4858: F, t1045: F, t606: F, t1053: F, t15670: F, t11262: F, t3127: F, t4824: F, t11671: F, t4954: F, t11998: F, t15822: F, t1086: F, t15669: F, t3090: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54123, t54127, t54148, t54316, t54388, t54397) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2550::<F>(t54122, t1011, t3252, t4574, t697, t11263, t4879, t43537, t53668, t11817, t4858, t1045, t606);
        let (t54404, t54414, t54471, t54492, t54500) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551::<F>(t1053, t15670, t11262, t3127, t4824, t11671, t4954, t11998, t15822, t1086, t15669, t3090);
    (t54123, t54127, t54148, t54316, t54388, t54397, t54404, t54414, t54471, t54492, t54500)
}
