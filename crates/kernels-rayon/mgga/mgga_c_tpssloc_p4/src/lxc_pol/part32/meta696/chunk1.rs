//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2162/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2162(t19882: f64, t22833: f64, t91114: f64, t91121: f64, t97202: f64, t97204: f64, t97206: f64, t97208: f64, t97210: f64, t97212: f64, t97214: f64, t97217: f64, t97219: f64, t97221: f64, t97223: f64, t97225: f64, t97227: f64, t97229: f64) -> f64 {
    let t97231 = t22833 * t19882;
    let t97233 = -t91114 + t91121 + t97202 / 256.0_f64 + t97204 / 768.0_f64 + t97206 / 192.0_f64 + t97208 / 192.0_f64 - t97210 / 768.0_f64 - t97212 / 1536.0_f64 + t97214 / 192.0_f64 + t97217 / 384.0_f64 - 7.0_f64 / 288.0_f64 * t97219 - t97221 / 1536.0_f64 - 5.0_f64 / 192.0_f64 * t97223 + t97225 / 192.0_f64 - t97227 / 768.0_f64 + t97229 / 192.0_f64 + t97231 / 384.0_f64;
    t97233
}
