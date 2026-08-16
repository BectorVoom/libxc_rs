//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 964/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk964(t1363: f64, t16211: f64, t1831: f64, t19834: f64, t19839: f64, t19841: f64, t19851: f64, t19904: f64, t20433: f64, t20442: f64, t20484: f64, t20508: f64, t20599: f64, t3803: f64, t5240: f64, t6427: f64, t6431: f64) -> f64 {
    let t20601 = -119.0_f64 / 4608.0_f64 * t16211 - t5240 * t6431 / 256.0_f64 + 5.0_f64 / 256.0_f64 * t5240 * t6427 - 5.0_f64 / 128.0_f64 * t1363 * t20433 - t19904 * t1831 / 256.0_f64 - 7.0_f64 / 1536.0_f64 * t19834 - 7.0_f64 / 16.0_f64 * t19839 + 7.0_f64 / 48.0_f64 * t19841 - t3803 * t20442 / 1024.0_f64 - 7.0_f64 / 768.0_f64 * t19851 + t20484 + t20508 + t20599;
    t20601
}
