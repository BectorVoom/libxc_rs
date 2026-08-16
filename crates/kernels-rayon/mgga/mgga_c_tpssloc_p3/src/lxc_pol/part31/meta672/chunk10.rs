//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2023/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2023(t80837: f64, t84514: f64, t84520: f64, t91244: f64, t91246: f64, t91247: f64, t93710: f64, t93711: f64, t93712: f64, t93715: f64, t93718: f64, t97352: f64, t97354: f64, t97359: f64, t97361: f64, t97363: f64, t97367: f64, t97372: f64) -> f64 {
    let t102705 = -t97352 / 192.0_f64 + 5.0_f64 / 192.0_f64 * t97354 + t91244 - t91246 + t91247 - t84514 + 5.0_f64 / 192.0_f64 * t97359 + 5.0_f64 / 96.0_f64 * t97361 - 7.0_f64 / 1152.0_f64 * t97363 - 0.13457585364713463618e-3_f64 * t97367 + 0.67287926823567318088e-4_f64 * t97372 + 0.20186378047070195426e-3_f64 * t80837 - t84520 + t93710 + t93711 + t93712 - t93715 - t93718;
    t102705
}
