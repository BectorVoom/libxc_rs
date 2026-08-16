//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2020/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2020(t91214: f64, t80761: f64, t80767: f64, t80769: f64, t80776: f64, t91183: f64, t91185: f64, t91187: f64, t91189: f64, t91192: f64, t91196: f64, t91200: f64, t91204: f64, t91206: f64, t91210: f64, t91212: f64, t91216: f64, t91218: f64) -> f64 {
    let t93674 = 7.0_f64 / 144.0_f64 * t91214;
    let t93681 = -5.0_f64 / 32.0_f64 * t91183 - t91185 / 768.0_f64 - t91187 / 384.0_f64 - t91189 / 768.0_f64 - t91192 / 96.0_f64 - t91196 / 2.0_f64 - 0.13565246047631171326e0_f64 * t91200 + 0.48447307312968469024e-2_f64 * t91204 - 0.63250651214153279003e-2_f64 * t91206 - 0.33913115119077928316e-1_f64 * t91210 - t91212 / 96.0_f64 - t93674 - t91216 / 768.0_f64 - t91218 / 384.0_f64 + 7.0_f64 / 72.0_f64 * t80761 - 0.27130492095262342653e0_f64 * t80767 + 0.16956557559538964158e-1_f64 * t80769 - 35.0_f64 / 54.0_f64 * t80776;
    t93681
}
