//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1065/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1065(t5612: f64, t7101: f64, t24218: f64, t24220: f64, t24221: f64, t25065: f64, t25077: f64, t25080: f64, t28357: f64, t28360: f64, t28362: f64, t28364: f64, t28366: f64, t28368: f64, t28370: f64, t28373: f64, t28376: f64) -> (f64, f64) {
    let t29010 = t7101 * t5612;
    let t29025 = 0.80745512188280781706e-3_f64 * t25065 - 0.40372756094140390853e-3_f64 * t28357 + t28360 / 768.0_f64 - t28362 / 192.0_f64 + 7.0_f64 / 144.0_f64 * t25077 - 7.0_f64 / 576.0_f64 * t25080 - t28364 / 768.0_f64 + t28366 / 384.0_f64 - t28368 / 384.0_f64 - t28370 / 768.0_f64 + t24218 - t24220 - 0.40372756094140390853e-3_f64 * t28373 + 0.80745512188280781706e-3_f64 * t28376 + t24221;
    (t29010, t29025)
}
