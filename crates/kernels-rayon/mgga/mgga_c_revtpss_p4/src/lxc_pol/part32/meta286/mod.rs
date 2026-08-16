//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1183;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta286(t235: f64, t9731: f64, t1389: f64, t3964: f64, t2735: f64, t546: f64, t1353: f64, t1412: f64, t808: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t159: f64, t216: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64, t212: f64, t225: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9732, t9735, t9736, t9737, t9739, t9741, t9742) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1183(t235, t9731, t1389, t3964, t2735, t546, t1353, t1412, t808, t1369, t2699, t1372);
        let (t9744, t9748, t9765, t9766, t9775) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1184(t3943, t794, t1412, t159, t216, t1408, t2482, t596, t3981, t212, t225, t816);
    (t9732, t9735, t9736, t9737, t9739, t9741, t9742, t9744, t9748, t9765, t9766, t9775)
}
