//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 735/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk735(t3721: f64, t812: f64, t1396: f64, t2401: f64, t253: f64, t3693: f64, t3695: f64, t3699: f64, t809: f64, t819: f64) -> (f64, f64) {
    let t3722 = t812 * t3721;
    let t3724 = -t1396 * t2401 + t253 * t3693 - t3695 * t819 + 2.0_f64 * t3699 * t809 - t3722 * t809;
    (t3722, t3724)
}
