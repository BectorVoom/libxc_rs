//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta286<F: Float>(t235: F, t9731: F, t1389: F, t3964: F, t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t159: F, t216: F, t1408: F, t2482: F, t596: F, t3981: F, t212: F, t225: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9732, t9735, t9736, t9737, t9739, t9741, t9742) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1183::<F>(t235, t9731, t1389, t3964, t2735, t546, t1353, t1412, t808, t1369, t2699, t1372);
        let (t9744, t9748, t9765, t9766, t9775) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1184::<F>(t3943, t794, t1412, t159, t216, t1408, t2482, t596, t3981, t212, t225, t816);
    (t9732, t9735, t9736, t9737, t9739, t9741, t9742, t9744, t9748, t9765, t9766, t9775)
}
