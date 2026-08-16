//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta405(t3089: f64, t42415: f64, t1087: f64, t11672: f64, t11711: f64, t1024: f64, t12003: f64, t10356: f64, t999: f64, t11744: f64, t3188: f64, t3181: f64, t675: f64, t1063: f64, t247: f64, t2853: f64, t11151: f64, t11725: f64, t1042: f64, t11653: f64, t11714: f64, t11748: f64, t15716: f64, t15728: f64, t15935: f64, t3101: f64, t3116: f64, t3127: f64, t3130: f64, t3182: f64, t41277: f64, t42001: f64, t283: f64, t2852: f64, t66: f64, t11951: f64, t3211: f64, t1025: f64, t3218: f64, t371: f64, t676: f64, t11804: f64, t11921: f64, t4837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42416, t42417, t42421, t42425, t42428, t42439, t42447) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1498(t3089, t42415, t1087, t11672, t11711, t1024, t12003, t10356, t999, t11744, t3188, t3181, t675);
        let t42456 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499(t1063, t247, t2853, t42447, t11151, t11725, t1042, t11653, t11714, t11748, t15716, t15728, t15935, t3101, t3116, t3127, t3130, t3182, t3188, t41277, t42001, t42421, t42425, t42428, t42439);
        let (t42472, t42477, t42481, t42487) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500(t283, t2852, t66, t11951, t3211, t1025, t3218, t371, t676, t11804, t11921, t247, t4837);
    (t42416, t42417, t42428, t42456, t42472, t42477, t42481, t42487)
}
