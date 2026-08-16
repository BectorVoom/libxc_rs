//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1047/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1047(t13760: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64, t1868: f64, t4010: f64, t1353: f64, t2661: f64) -> (f64, f64, f64, f64) {
    let t13762 = 0.10164000561857065645e-3_f64 * t3978 * t13760;
    let t13763 = t3989 * t5614;
    let t13765 = t9765 * t5622;
    let t13767 = t1408 * t240;
    let t13768 = t4010 * t1868;
    let t13769 = t13768 * t1353;
    let t13770 = t13767 * t13769;
    let t13772 = 0.28582678745379824648e-3_f64 * t2661 * t13770;
    (t13762, t13763, t13765, t13772)
}
