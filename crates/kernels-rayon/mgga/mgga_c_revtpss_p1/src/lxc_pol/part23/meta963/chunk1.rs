//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3256/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3256(t2661: f64, t3992: f64, t6869: f64, t74026: f64, t13999: f64, t22843: f64, t22854: f64, t3989: f64, t221: f64, t22852: f64, t3978: f64, t9921: f64) -> (f64, f64, f64, f64) {
    let t85741 = t2661 * t3992 * t74026 * t6869;
    let t85752 = t13999 * t22843;
    let t85764 = t3989 * t22854;
    let t85776 = t221 * t22852;
    let t85778 = t3978 * t9921 * t85776;
    (t85741, t85752, t85764, t85778)
}
