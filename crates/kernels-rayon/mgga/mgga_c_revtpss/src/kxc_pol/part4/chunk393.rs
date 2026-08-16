//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 393/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk393(t1221: f64, t1222: f64, t1227: f64, t1231: f64, t1235: f64, t1238: f64, t1247: f64, t1252: f64, t1258: f64, t1261: f64, t1266: f64, t484: f64) -> f64 {
    let t1269 = t1221 - t1222 * t1227 / 288.0_f64 + 0.21437009059034868486e-3_f64 * t1231 * t484 - 0.21437009059034868486e-3_f64 * t1235 * t1238 + 0.21437009059034868486e-3_f64 * t1247 * t1252 + t1258 - 0.14291339372689912324e-3_f64 * t1261 * t1266;
    t1269
}
