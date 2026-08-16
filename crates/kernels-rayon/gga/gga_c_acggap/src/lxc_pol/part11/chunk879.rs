//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 879/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk879(t30374: f64, t7499: f64, t7315: f64, t7828: f64, t130: f64, t1977: f64, t7858: f64, t7861: f64, t2025: f64, t7852: f64, t593: f64, t7510: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30375 = t30374 * t7499;
    let t30387 = t7315 * t7828;
    let t30394 = t130 * t1977;
    let t30396 = t30394 * t7858 * t7861;
    let t30397 = 5.0_f64 / 128.0_f64 * t30396;
    let t30398 = t7852 * t2025;
    let t30400 = t593 * t7510;
    (t30375, t30387, t30394, t30397, t30398, t30400)
}
