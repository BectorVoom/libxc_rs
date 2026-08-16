//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1848/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848(t87071: f64, t92516: f64, t116: f64, t117: f64, t1916: f64, t1918: f64, t22633: f64, t25055: f64, t25063: f64, t25066: f64, t25069: f64, t572: f64, t573: f64, t5801: f64, t5883: f64, t5920: f64, t6941: f64, t6945: f64, t6948: f64, t87051: f64, t87237: f64, param_d: f64) -> (f64, f64) {
    let t92517 = t87071 + t92516;
    let t92552 = 18.0_f64 * t116 * t572 * t87237 + 3.0_f64 * t117 * t572 * t87051 + 24.0_f64 * t22633 * t572 * t5801 + 36.0_f64 * t572 * t5883 * t5920 + t573 * t92517 * param_d + 24.0_f64 * t1916 * t25063 + 72.0_f64 * t1916 * t25066 + 12.0_f64 * t1916 * t25069 + 12.0_f64 * t1918 * t25055 + 36.0_f64 * t6941 * t6945 + 18.0_f64 * t6941 * t6948;
    (t92517, t92552)
}
