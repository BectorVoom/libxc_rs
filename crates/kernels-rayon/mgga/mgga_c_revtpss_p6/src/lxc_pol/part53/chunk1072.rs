//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1072/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1072(t117: f64, t34418: f64, t118: f64, t1843: f64, t1932: f64, t2163: f64, t33600: f64, t33603: f64, t33605: f64, t33650: f64, t33654: f64, t34383: f64, t34394: f64, t34400: f64, t34401: f64, t508: f64, t7725: f64, t8233: f64, t8741: f64) -> (f64, f64) {
    let t34419 = t34418 * t117;
    let t34422 = -t118 * t34394 - t1843 * t8741 - t1932 * t8233 - t2163 * t7725 - t34419 * t508 - 2.0_f64 * t33600 - 2.0_f64 * t33603 - 2.0_f64 * t33605 - t33650 - t33654 - 2.0_f64 * t34383 + t34400 + t34401;
    (t34419, t34422)
}
