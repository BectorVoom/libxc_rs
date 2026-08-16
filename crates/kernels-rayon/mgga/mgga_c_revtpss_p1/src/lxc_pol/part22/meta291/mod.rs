//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta291(t1353: f64, t1412: f64, t808: f64, t9736: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t3946: f64, t159: f64, t216: f64, t3989: f64, t4014: f64, t221: f64, t3889: f64, t3979: f64, t3978: f64, t1408: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9738, t9739, t9741) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1706(t1353, t1412, t808, t9736, t1369, t2699);
        let (t9742, t9744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1707(t1372, t9741, t3943, t794);
        let (t9745, t9747, t9748, t9753, t9761, t9762, t9765) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1708(t3946, t9744, t1412, t159, t216, t3989, t4014, t221, t3889, t3979, t3978, t1408, t2482, t596);
    (t9738, t9739, t9741, t9742, t9744, t9745, t9747, t9748, t9753, t9761, t9762, t9765)
}
