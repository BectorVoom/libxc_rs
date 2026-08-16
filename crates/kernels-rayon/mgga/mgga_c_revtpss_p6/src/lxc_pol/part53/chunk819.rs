//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 819/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk819(t2035: f64, t8764: f64, t118: f64, t1932: f64, t2007: f64, t2127: f64, t2163: f64, t508: f64, t569: f64, t8449: f64, t8456: f64, t8458: f64, t8463: f64, t8569: f64, t8597: f64, t8601: f64, t8741: f64, t8743: f64, t8750: f64, t8756: f64, t8761: f64) -> f64 {
    let t8765 = t8764 * t2035;
    let t8766 = -t118 * t8756 - t1932 * t2163 - t2007 * t2127 - t508 * t8741 + t569 * t8761 - 2.0_f64 * t8449 - t8456 - 2.0_f64 * t8458 - t8463 + t8569 + t8597 - t8601 - 2.0_f64 * t8743 - 2.0_f64 * t8750 + t8765;
    t8766
}
