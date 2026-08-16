//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1235/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1235(t127384: f64, t127385: f64, t127393: f64, t127395: f64, t127397: f64, t127399: f64, t127401: f64, t127403: f64, t127405: f64, t1843: f64, t1932: f64, t29337: f64, t32815: f64, t5517: f64, t6983: f64, t8233: f64, t8741: f64) -> f64 {
    let t129519 = -t1843 * t32815 - t1932 * t29337 - t5517 * t8741 - t6983 * t8233 - t127384 - t127385 - 2.0_f64 * t127393 - 2.0_f64 * t127395 - 2.0_f64 * t127397 - 2.0_f64 * t127399 - 2.0_f64 * t127401 - 2.0_f64 * t127403 - 2.0_f64 * t127405;
    t129519
}
