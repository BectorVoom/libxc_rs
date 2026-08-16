//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3266/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266(t30: f64, t48294: f64, t1317: f64, t22790: f64, t1320: f64, t13550: f64, t13553: f64, t18280: f64, t21906: f64, t2255: f64, t22670: f64, t22769: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t605: f64, t76396: f64, t85406: f64, t85409: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t85928 = 360.0_f64 * t48294;
    let t85929 = t1317 * t22790;
    let t85930 = 4.0_f64 * t85929;
    let t85931 = t1320 * t22790;
    let t85932 = 4.0_f64 * t85931;
    let t85950 = piecewise3(t31, 0.0_f64, 40.0_f64 / 81.0_f64 * t47025 * t22769 * t605 - 16.0_f64 / 9.0_f64 * t21906 * t2255 - 8.0_f64 / 9.0_f64 * t13550 * t85406 + 8.0_f64 / 3.0_f64 * t13553 * t85409 + 4.0_f64 / 3.0_f64 * t5549 * t18280 + 4.0_f64 / 9.0_f64 * t3833 * t22670 * t605 + 4.0_f64 / 3.0_f64 * t513 * t76396);
    (t85928, t85930, t85932, t85950)
}
