//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1123/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1123(t14459: f64, t4496: f64, t959: f64, t17194: f64, t17197: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64, t17929: f64) -> (f64, f64) {
    let t17930 = t4496 * t14459;
    let t17932 = 0.34631718211362927518e2_f64 * t959 * t17930;
    let t17933 = t17194 + t17197 - t17209 - t17301 - t17303 - t17306 + t17561 - t17563 - t17568 + t17372 + t17374 - t17377 + t17379 + t17425 + t17427 + t17929 - t17932;
    (t17932, t17933)
}
