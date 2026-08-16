//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2548;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta757(t53875: f64, t225: f64, t53014: f64, t3091: f64, t43240: f64, t4787: f64, t3105: f64, t4857: f64, t1012: f64, t43222: f64, t15711: f64, t3188: f64, t11821: f64, t140: f64, t42793: f64, t4892: f64, t4895: f64, t4899: f64, t4901: f64, t1011: f64, t1655: f64, t2438: f64, t1014: f64, t4579: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53876, t53877, t53901, t53926, t53944, t53955) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2548(t53875, t225, t53014, t3091, t43240, t4787, t3105, t4857, t1012, t43222, t15711, t3188);
        let (t53972, t54037, t54079, t54118, t54122) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2549(t11821, t140, t42793, t4892, t4895, t4899, t4901, t1011, t1655, t2438, t1014, t4579, t697);
    (t53876, t53877, t53901, t53926, t53944, t53955, t53972, t54037, t54079, t54118, t54122)
}
