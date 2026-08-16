//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1978/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1978(t87233: f64, t87243: f64, t87247: f64, t87255: f64, t81764: f64, t81770: f64, t81772: f64, t81785: f64, t87222: f64, t87224: f64, t87226: f64, t87235: f64, t87241: f64, t87245: f64, t87249: f64, t87251: f64, t87253: f64, t87257: f64) -> f64 {
    let t92590 = 0.26915170729426927236e-3_f64 * t87233;
    let t92597 = 119.0_f64 / 3456.0_f64 * t87243;
    let t92599 = 7.0_f64 / 576.0_f64 * t87247;
    let t92603 = 7.0_f64 / 576.0_f64 * t87255;
    let t92605 = -t87222 / 192.0_f64 - t87224 / 96.0_f64 - t87226 / 192.0_f64 - t92590 + 5.0_f64 / 192.0_f64 * t87235 - 119.0_f64 / 432.0_f64 * t81764 + 7.0_f64 / 144.0_f64 * t81770 + 7.0_f64 / 288.0_f64 * t81772 - 0.80745512188280781706e-3_f64 * t81785 + 5.0_f64 / 96.0_f64 * t87241 - t92597 - t87245 / 768.0_f64 + t92599 - t87249 / 768.0_f64 - t87251 / 384.0_f64 - t87253 / 768.0_f64 + t92603 - 5.0_f64 / 32.0_f64 * t87257;
    t92605
}
