//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1392/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1392(t107183: f64, t107186: f64, t107189: f64, t107198: f64, t107205: f64, t80900: f64, t80957: f64, t80971: f64, t91394: f64, t91398: f64, t91400: f64, t97394: f64, t97400: f64, t97402: f64, t97404: f64, t97427: f64, t97431: f64, t97439: f64, t97444: f64, t97463: f64) -> f64 {
    let t107208 = 0.12111826828242117256e-2_f64 * t107183 - t80900 - 0.20186378047070195427e-3_f64 * t107186 + 3.0_f64 / 16.0_f64 * t107189 + 7.0_f64 / 48.0_f64 * t97394 - 0.84782787797694820794e-2_f64 * t97400 - 119.0_f64 / 2304.0_f64 * t91394 - 7.0_f64 / 16.0_f64 * t97402 - 0.17804385437515912366e0_f64 * t97404 - 0.42391393898847410397e-2_f64 * t97427 + 0.60559134141210586281e-3_f64 * t97431 - t107198 / 512.0_f64 + 0.25434836339308446238e-1_f64 * t97439 - 35.0_f64 / 72.0_f64 * t91398 - 0.2034786907144675699e0_f64 * t91400 + 0.42391393898847410397e-2_f64 * t97444 + t107205 / 1536.0_f64 - t80957 + t80971 + 0.42391393898847410397e-2_f64 * t97463;
    t107208
}
