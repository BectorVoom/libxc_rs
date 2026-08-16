//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2044/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2044(t102009: f64, t102058: f64, t102791: f64, t103873: f64, t103917: f64, t103956: f64, t103999: f64, t104038: f64, t101725: f64, t101761: f64, t117: f64, t13514: f64, t1459: f64, t1518: f64, t18190: f64, t18204: f64, t18208: f64, t18211: f64, t1916: f64, t2113: f64, t2115: f64, t26733: f64, t26740: f64, t28974: f64, t28987: f64, t28990: f64, t4162: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t7547: f64, t7553: f64, t7557: f64, t8118: f64, t96640: f64, param_d: f64) -> (f64, f64) {
    let t104041 = t102009 + t102058 + t102791 + t103873 + t103917 + t103956 + t103999 + t104038;
    let t104054 = 12.0_f64 * t572 * t101725 * t1518 + 12.0_f64 * t572 * t28974 * t4292 + 6.0_f64 * t572 * t7553 * t13514 + 12.0_f64 * t1459 * t28987 + 3.0_f64 * t1916 * t26740 + 6.0_f64 * t1459 * t28990 + 12.0_f64 * t7547 * t5802 + 6.0_f64 * t572 * t96640 * t1518 + 12.0_f64 * t572 * t26733 * t4292 + 12.0_f64 * t2113 * t18208 + 3.0_f64 * t572 * t117 * t101761 + 6.0_f64 * t2113 * t18204 + param_d * t104041 * t573 + 6.0_f64 * t2113 * t18211 + 6.0_f64 * t5795 * t7557 + 6.0_f64 * t8118 * t4162 + 3.0_f64 * t18190 * t2115 + 6.0_f64 * t7547 * t5805;
    (t104041, t104054)
}
