//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1995/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1995(t108879: f64, t2047: f64, t101237: f64, t101240: f64, t101850: f64, t108872: f64, t108876: f64, t108945: f64, t108952: f64, t2048: f64, t26175: f64, t28154: f64, t28628: f64, t29513: f64, t29551: f64, t7352: f64, t92568: f64, t95253: f64, t95255: f64, t95316: f64) -> f64 {
    let t109911 = t2047 * t108879;
    let t109918 = -2.0_f64 / 3.0_f64 * t108945 * t2048 - 2.0_f64 / 3.0_f64 * t29551 * t7352 + t108952 * t2048 / 3.0_f64 + t29513 * t7352 / 3.0_f64 - t95253 + 88.0_f64 / 27.0_f64 * t95255 + 20.0_f64 / 3.0_f64 * t28154 * t101850 - 70.0_f64 * t95316 * t108872 + 20.0_f64 * t26175 * t108876 - 20.0_f64 * t92568 * t109911 + 20.0_f64 / 3.0_f64 * t101237 * t28628 + 20.0_f64 / 3.0_f64 * t101240 * t28628;
    t109918
}
