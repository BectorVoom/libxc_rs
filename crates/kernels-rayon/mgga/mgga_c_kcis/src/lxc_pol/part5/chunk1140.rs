//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1140/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1140(t1001: f64, t19180: f64, t286: f64, t14394: f64, t14423: f64, t14427: f64, t14439: f64, t14442: f64, t14446: f64, t14450: f64, t14455: f64, t19166: f64, t19173: f64, t19176: f64, t285: f64, t9614: f64) -> f64 {
    let t19181 = t1001 * t19180;
    let t19182 = t286 * t19181;
    let t19186 = -t14394 * t19166 / 108.0_f64 + t9614 / 432.0_f64 + t14423 / 216.0_f64 - t14427 + t14439 + t14394 * t19173 / 72.0_f64 + t14394 * t19176 / 72.0_f64 - t285 * t19182 / 96.0_f64 - t14442 - t14446 + t14450 + t14455 / 216.0_f64;
    t19186
}
