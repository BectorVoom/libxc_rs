//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1445;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1446;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1447;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta257<F: Float>(t1353: F, t1412: F, t808: F, t9736: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t159: F, t216: F, t1408: F, t2482: F, t596: F, t3981: F, t212: F, t225: F, t816: F, t3995: F, t2681: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9738, t9739, t9741, t9742, t9744) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1445::<F>(t1353, t1412, t808, t9736, t1369, t2699, t1372, t3943, t794);
        let (t9747, t9748, t9765) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1446::<F>(t1412, t159, t216, t1408, t2482, t596);
        let (t9766, t9775) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1447::<F>(t3981, t9765, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1448::<F>(t3995, t9775, t1408, t2681, t820);
    (t9738, t9739, t9741, t9742, t9744, t9747, t9748, t9765, t9766, t9775, t9776, t9779)
}
