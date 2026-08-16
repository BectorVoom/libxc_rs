//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1008/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1008(t10894: f64, t812: f64, t10819: f64, t10821: f64, t10833: f64, t10837: f64, t10841: f64, t1396: f64, t2401: f64, t2408: f64, t2426: f64, t253: f64, t3695: f64, t3699: f64, t3722: f64, t809: f64, t819: f64, t8339: f64) -> (f64, f64) {
    let t10895 = t812 * t10894;
    let t10897 = t10819 * t253 - 2.0_f64 * t10821 * t819 - 6.0_f64 * t10833 * t809 + 4.0_f64 * t10837 * t809 + 2.0_f64 * t10841 * t809 - t10895 * t809 - t1396 * t8339 + 4.0_f64 * t2401 * t3699 - 2.0_f64 * t2401 * t3722 + 2.0_f64 * t2408 * t3695 - t2426 * t3695;
    (t10895, t10897)
}
