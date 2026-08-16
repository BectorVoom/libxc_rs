//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1148/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1148(t33: f64, t1711: f64, t9350: f64, t2: f64, t3841: f64, t1113: f64, t580: f64, t22: f64, t3351: f64, t3842: f64, t516: f64, t5557: f64, t5560: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t13565 = t9350 * t1711;
    let t13568 = t3841 * t2;
    let t13569 = t580 * t1113;
    let t13579 = piecewise3(t34, 0.0_f64, -8.0_f64 / 27.0_f64 * t13565 * t3842 - 16.0_f64 / 9.0_f64 * t13568 * t13569 + 4.0_f64 / 9.0_f64 * t5557 * t3351 - 8.0_f64 / 3.0_f64 * t516 * t580 + 8.0_f64 * t5560 * t22);
    (t13569, t13579)
}
