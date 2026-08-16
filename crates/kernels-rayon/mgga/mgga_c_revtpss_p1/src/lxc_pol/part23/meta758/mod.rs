//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2550;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta758(t54122: f64, t1011: f64, t3252: f64, t4574: f64, t697: f64, t11263: f64, t4879: f64, t43537: f64, t53668: f64, t11817: f64, t4858: f64, t1045: f64, t606: f64, t1053: f64, t15670: f64, t11262: f64, t3127: f64, t4824: f64, t11671: f64, t4954: f64, t11998: f64, t15822: f64, t1086: f64, t15669: f64, t3090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54123, t54127, t54148, t54316, t54388, t54397) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2550(t54122, t1011, t3252, t4574, t697, t11263, t4879, t43537, t53668, t11817, t4858, t1045, t606);
        let (t54404, t54414, t54471, t54492, t54500) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551(t1053, t15670, t11262, t3127, t4824, t11671, t4954, t11998, t15822, t1086, t15669, t3090);
    (t54123, t54127, t54148, t54316, t54388, t54397, t54404, t54414, t54471, t54492, t54500)
}
