//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1212/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1212(t107183: f64, t107186: f64, t107189: f64, t107198: f64, t107205: f64, t84536: f64, t84555: f64, t84558: f64, t91394: f64, t91398: f64, t91400: f64, t97394: f64, t97400: f64, t97402: f64, t97404: f64, t97427: f64, t97431: f64, t97439: f64, t97444: f64, t97463: f64) -> f64 {
    let t107860 = 0.24223653656484234512e-2_f64 * t107183 - t84536 - 0.40372756094140390853e-3_f64 * t107186 + 3.0_f64 / 8.0_f64 * t107189 + 7.0_f64 / 24.0_f64 * t97394 - 0.16956557559538964158e-1_f64 * t97400 - 119.0_f64 / 1152.0_f64 * t91394 - 7.0_f64 / 8.0_f64 * t97402 - 0.35608770875031824732e0_f64 * t97404 - 0.84782787797694820791e-2_f64 * t97427 + 0.12111826828242117256e-2_f64 * t97431 - t107198 / 256.0_f64 + 0.50869672678616892474e-1_f64 * t97439 - 35.0_f64 / 36.0_f64 * t91398 - 0.4069573814289351398e0_f64 * t91400 + 0.84782787797694820791e-2_f64 * t97444 + t107205 / 768.0_f64 - t84555 + t84558 + 0.84782787797694820791e-2_f64 * t97463;
    t107860
}
