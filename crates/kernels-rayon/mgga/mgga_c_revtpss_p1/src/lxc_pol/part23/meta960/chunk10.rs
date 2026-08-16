//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3243/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3243(t4245: f64, t5883: f64, t1310: f64, t1502: f64, t18220: f64, t1843: f64, t1911: f64, t21658: f64, t21881: f64, t21882: f64, t22525: f64, t22639: f64, t22747: f64, t27123: f64, t30138: f64, t4246: f64, t4248: f64, t4292: f64, t4293: f64, t508: f64, t5517: f64, t5877: f64, t5884: f64, t5921: f64, t651: f64, t6765: f64) -> (f64, f64) {
    let t85329 = t4245 * t5883;
    let t85343 = -6.0_f64 * t1843 * t21881 * t651 - 6.0_f64 * t4292 * t651 * t6765 - 6.0_f64 * t1310 * t22639 - t1310 * t22747 - 3.0_f64 * t1502 * t21658 - 6.0_f64 * t18220 * t1843 + 3.0_f64 * t1911 * t22525 - 6.0_f64 * t21882 * t4248 - 6.0_f64 * t27123 * t5921 - 12.0_f64 * t30138 * t4293 - 3.0_f64 * t4246 * t6765 - 6.0_f64 * t508 * t85329 - 3.0_f64 * t5517 * t5877 - 6.0_f64 * t5517 * t5884;
    (t85329, t85343)
}
