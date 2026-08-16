//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta405<F: Float>(t3089: F, t42415: F, t1087: F, t11672: F, t11711: F, t1024: F, t12003: F, t10356: F, t999: F, t11744: F, t3188: F, t3181: F, t675: F, t1063: F, t247: F, t2853: F, t11151: F, t11725: F, t1042: F, t11653: F, t11714: F, t11748: F, t15716: F, t15728: F, t15935: F, t3101: F, t3116: F, t3127: F, t3130: F, t3182: F, t41277: F, t42001: F, t283: F, t2852: F, t66: F, t11951: F, t3211: F, t1025: F, t3218: F, t371: F, t676: F, t11804: F, t11921: F, t4837: F) -> (F, F, F, F, F, F, F, F) {
        let (t42416, t42417, t42421, t42425, t42428, t42439, t42447) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1498::<F>(t3089, t42415, t1087, t11672, t11711, t1024, t12003, t10356, t999, t11744, t3188, t3181, t675);
        let t42456 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499::<F>(t1063, t247, t2853, t42447, t11151, t11725, t1042, t11653, t11714, t11748, t15716, t15728, t15935, t3101, t3116, t3127, t3130, t3182, t3188, t41277, t42001, t42421, t42425, t42428, t42439);
        let (t42472, t42477, t42481, t42487) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500::<F>(t283, t2852, t66, t11951, t3211, t1025, t3218, t371, t676, t11804, t11921, t247, t4837);
    (t42416, t42417, t42428, t42456, t42472, t42477, t42481, t42487)
}
