//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1114/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1114(t13716: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64) -> (f64, f64, f64, f64, f64) {
    let t13756 = t1414 * t828 * t13716;
    let t13760 = t3979 * t221 * t5591;
    let t13762 = 0.10164000561857065645e-3_f64 * t3978 * t13760;
    let t13763 = t3989 * t5614;
    let t13765 = t9765 * t5622;
    let t13767 = t1408 * t240;
    (t13756, t13762, t13763, t13765, t13767)
}
