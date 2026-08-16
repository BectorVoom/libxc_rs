//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2236/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2236(t80767: f64, t80776: f64, t80761: f64, t80769: f64, t91183: f64, t91185: f64, t91187: f64, t91189: f64, t91192: f64, t91196: f64, t91200: f64, t91204: f64, t91206: f64, t91210: f64, t91212: f64, t91215: f64, t91216: f64, t91218: f64) -> f64 {
    let t91221 = 0.13565246047631171327e0_f64 * t80767;
    let t91223 = 35.0_f64 / 108.0_f64 * t80776;
    let t91224 = -5.0_f64 / 64.0_f64 * t91183 - t91185 / 1536.0_f64 - t91187 / 768.0_f64 - t91189 / 1536.0_f64 - t91192 / 192.0_f64 - t91196 / 4.0_f64 - 0.67826230238155856634e-1_f64 * t91200 + 0.24223653656484234512e-2_f64 * t91204 - 0.31625325607076639502e-2_f64 * t91206 - 0.16956557559538964158e-1_f64 * t91210 - t91212 / 192.0_f64 - t91215 - t91216 / 1536.0_f64 - t91218 / 768.0_f64 + 7.0_f64 / 144.0_f64 * t80761 - t91221 + 0.84782787797694820794e-2_f64 * t80769 - t91223;
    t91224
}
