//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1571/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1571(t1882: f64, t6843: f64, t22881: f64, t9962: f64, t6869: f64, t73856: f64, t9816: f64, t9818: f64, t2661: f64, t3992: f64, t74026: f64, t13999: f64, t22843: f64) -> (f64, f64, f64, f64, f64) {
    let t85659 = t6843 * t1882;
    let t85705 = t9962 * t22881;
    let t85735 = t9816 * t9818 * t73856 * t6869;
    let t85741 = t2661 * t3992 * t74026 * t6869;
    let t85752 = t13999 * t22843;
    (t85659, t85705, t85735, t85741, t85752)
}
