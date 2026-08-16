//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1420;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta383(t3173: f64, t4879: f64, t4866: f64, t73: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t3252: f64, t4574: f64, t1012: f64, t11821: f64, t11922: f64, t4906: f64, t3115: f64, t4895: f64, t4892: f64, t4886: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15942, t15957, t15984, t15986, t15990, t15993) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1420(t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140, t4579, t1011, t3252);
        let (t15996, t16012, t16035, t16037, t16055, t16057, t16060) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1421(t15993, t4574, t1011, t1012, t11821, t11922, t4906, t3115, t4895, t4892, t140, t4886);
    (t15942, t15957, t15984, t15986, t15990, t15996, t16012, t16035, t16037, t16055, t16057, t16060)
}
