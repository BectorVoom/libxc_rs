//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2019/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2019(t93633: f64, t93636: f64, t97202: f64, t97204: f64, t97206: f64, t97208: f64, t97210: f64, t97212: f64, t97214: f64, t97217: f64, t97219: f64, t97221: f64, t97223: f64, t97225: f64, t97227: f64, t97229: f64, t97231: f64) -> f64 {
    let t102647 = -t93633 + t93636 + t97202 / 128.0_f64 + t97204 / 384.0_f64 + t97206 / 96.0_f64 + t97208 / 96.0_f64 - t97210 / 384.0_f64 - t97212 / 768.0_f64 + t97214 / 96.0_f64 + t97217 / 192.0_f64 - 7.0_f64 / 144.0_f64 * t97219 - t97221 / 768.0_f64 - 5.0_f64 / 96.0_f64 * t97223 + t97225 / 96.0_f64 - t97227 / 384.0_f64 + t97229 / 96.0_f64 + t97231 / 192.0_f64;
    t102647
}
