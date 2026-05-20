//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta273<F: Float>(t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t159: F, t216: F, t3989: F, t4014: F, t221: F, t3889: F, t3979: F, t3978: F, t1408: F, t2482: F, t596: F, t3981: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9736, t9739, t9741, t9742, t9744, t9745, t9747) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1008::<F>(t2735, t546, t1353, t1412, t808, t1369, t2699, t1372, t3943, t794, t3946, t159);
        let (t9748, t9753, t9762, t9765, t9766) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1009::<F>(t216, t9747, t3989, t4014, t221, t3889, t3979, t3978, t1408, t2482, t596, t3981);
    (t9736, t9739, t9741, t9742, t9744, t9745, t9748, t9753, t9762, t9765, t9766)
}
