//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3106/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3106(t17763: f64, t3636: f64, t13085: f64, t5391: f64, t3568: f64, t606: f64, t12881: f64, t5381: f64, t127: f64, t12866: f64, t17650: f64, t5296: f64) -> (f64, f64, f64, f64, f64) {
    let t57075 = t17763 * t3636;
    let t57077 = t5391 * t13085;
    let t57083 = t3568 * t606;
    let t57094 = t5381 * t12881;
    let t57098 = t12866 * t127 * t5296 * t17650;
    (t57075, t57077, t57083, t57094, t57098)
}
