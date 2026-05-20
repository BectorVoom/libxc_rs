//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2548;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta757<F: Float>(t53875: F, t225: F, t53014: F, t3091: F, t43240: F, t4787: F, t3105: F, t4857: F, t1012: F, t43222: F, t15711: F, t3188: F, t11821: F, t140: F, t42793: F, t4892: F, t4895: F, t4899: F, t4901: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53876, t53877, t53901, t53926, t53944, t53955) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2548::<F>(t53875, t225, t53014, t3091, t43240, t4787, t3105, t4857, t1012, t43222, t15711, t3188);
        let (t53972, t54037, t54079, t54118, t54122) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549::<F>(t11821, t140, t42793, t4892, t4895, t4899, t4901, t1011, t1655, t2438, t1014, t4579, t697);
    (t53876, t53877, t53901, t53926, t53944, t53955, t53972, t54037, t54079, t54118, t54122)
}
