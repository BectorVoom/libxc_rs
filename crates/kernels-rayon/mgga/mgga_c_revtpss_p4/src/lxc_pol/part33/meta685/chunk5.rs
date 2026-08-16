//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2268/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2268(t109172: f64, t109176: f64, t109178: f64, t109180: f64, t109182: f64, t109194: f64, t109196: f64, t109198: f64, t109202: f64, t109262: f64, t109266: f64, t109268: f64, t109271: f64, t109274: f64, t111809: f64, t113002: f64, t118: f64, t18220: f64, t2163: f64, t29427: f64, t4297: f64, t5884: f64, t6934: f64, t7683: f64, t7687: f64) -> f64 {
    let t113012 = -t109172 - t118 * (t111809 + t113002) - 2.0_f64 * t18220 * t2163 - 2.0_f64 * t5884 * t7683 + t109176 - t109178 + t109180 + t7687 * t6934 + t109182 - 4.0_f64 * t29427 * t4297 - t109194 - t109196 - t109198 + t109202 + t109262 + t109266 - t109268 + t109271 - t109274;
    t113012
}
