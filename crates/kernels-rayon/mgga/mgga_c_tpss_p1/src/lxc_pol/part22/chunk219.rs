//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 219/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk219(t177: f64, t713: f64, t662: f64, t664: f64, t668: f64, t673: f64) -> (f64, f64) {
    let t714 = t177 * t713;
    let t719 = -0.86308333333333333334e0_f64 * t662 - 0.301925e0_f64 * t664 - 0.5501625e-1_f64 * t668 - 0.82785e-1_f64 * t673;
    (t714, t719)
}
