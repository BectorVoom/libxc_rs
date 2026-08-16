//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1328/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1328(t25082: f64, t30122: f64, t33651: f64, t18245: f64, t7742: f64, t114378: f64, t1937: f64, t30138: f64, t7735: f64, t29576: f64, t7898: f64, t30128: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114415 = 18.0_f64 * t25082 * t33651 * t30122;
    let t114417 = 6.0_f64 * t18245 * t7742;
    let t114419 = 6.0_f64 * t114378 * t1937;
    let t114421 = 12.0_f64 * t30138 * t7735;
    let t114427 = 6.0_f64 * t7898 * t29576;
    let t114434 = 12.0_f64 * t30138 * t7742;
    let t114436 = 6.0_f64 * t4248 * t30128;
    (t114415, t114417, t114419, t114421, t114427, t114434, t114436)
}
