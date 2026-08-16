//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1502/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1502(t117579: f64, t117622: f64, t117666: f64, t117711: f64, t117381: f64, t117385: f64, t1518: f64, t18190: f64, t18204: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t31100: f64, t31118: f64, t31121: f64, t31358: f64, t4162: f64, t4165: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t8289: f64, t8296: f64, t8299: f64, t8377: f64, param_d: f64) -> (f64, f64) {
    let t117713 = t117579 + t117622 + t117666 + t117711;
    let t117720 = 6.0_f64 * t117381 * t1518 * t572 + 12.0_f64 * t117385 * t1518 * t572 + t117713 * t573 * param_d + 12.0_f64 * t31358 * t4292 * t572 + 3.0_f64 * t18190 * t2189 + 6.0_f64 * t18204 * t2187 + 12.0_f64 * t18208 * t2187 + 6.0_f64 * t18211 * t2187 + 3.0_f64 * t18214 * t2187 + 12.0_f64 * t1916 * t31118 + 6.0_f64 * t1916 * t31121 + 3.0_f64 * t1918 * t31100 + 6.0_f64 * t4162 * t8377 + 3.0_f64 * t4165 * t8377 + 12.0_f64 * t5795 * t8296 + 6.0_f64 * t5795 * t8299 + 6.0_f64 * t5805 * t8289;
    (t117713, t117720)
}
