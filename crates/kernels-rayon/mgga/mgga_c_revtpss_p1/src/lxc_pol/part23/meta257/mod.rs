//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1445;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1446;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1447;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta257(t1353: f64, t1412: f64, t808: f64, t9736: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t159: f64, t216: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64, t212: f64, t225: f64, t816: f64, t3995: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9738, t9739, t9741, t9742, t9744) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1445(t1353, t1412, t808, t9736, t1369, t2699, t1372, t3943, t794);
        let (t9747, t9748, t9765) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1446(t1412, t159, t216, t1408, t2482, t596);
        let (t9766, t9775) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1447(t3981, t9765, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1448(t3995, t9775, t1408, t2681, t820);
    (t9738, t9739, t9741, t9742, t9744, t9747, t9748, t9765, t9766, t9775, t9776, t9779)
}
