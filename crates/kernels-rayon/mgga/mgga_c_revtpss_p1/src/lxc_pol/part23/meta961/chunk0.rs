//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3245/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3245(t30: f64, t46281: f64, t46286: f64, t5824: f64, t605: f64, t580: f64, t1344: f64, t13687: f64, t13690: f64, t18280: f64, t21944: f64, t2255: f64, t22670: f64, t22769: f64, t3874: f64, t46310: f64, t5574: f64, t76396: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t85390 = 60.0_f64 * t46281;
    let t85391 = 0.5848223622634646207e0_f64 * t46286;
    let t85406 = t5824 * t605;
    let t85409 = t580 * t5824;
    let t85420 = piecewise3(t31, 0.0_f64, -56.0_f64 / 81.0_f64 * t46310 * t22769 * t605 + 16.0_f64 / 9.0_f64 * t21944 * t2255 + 8.0_f64 / 9.0_f64 * t13687 * t85406 - 4.0_f64 / 3.0_f64 * t13690 * t85409 - 2.0_f64 / 3.0_f64 * t5574 * t18280 - 2.0_f64 / 9.0_f64 * t3874 * t22670 * t605 + 2.0_f64 / 3.0_f64 * t1344 * t76396);
    (t85390, t85391, t85406, t85409, t85420)
}
