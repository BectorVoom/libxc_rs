//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1011;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta273(t2735: f64, t546: f64, t1353: f64, t1412: f64, t808: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t3946: f64, t159: f64, t216: f64, t3989: f64, t4014: f64, t221: f64, t3889: f64, t3979: f64, t3978: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9736, t9739, t9741, t9742, t9744, t9745, t9747) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1011(t2735, t546, t1353, t1412, t808, t1369, t2699, t1372, t3943, t794, t3946, t159);
        let (t9748, t9753, t9762, t9765, t9766) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1012(t216, t9747, t3989, t4014, t221, t3889, t3979, t3978, t1408, t2482, t596, t3981);
    (t9736, t9739, t9741, t9742, t9744, t9745, t9748, t9753, t9762, t9765, t9766)
}
