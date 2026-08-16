//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2222/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2222(t15775: f64, t7132: f64, t100054: f64, t3299: f64, t100030: f64, t15158: f64, t15586: f64, t15611: f64, t15697: f64, t16027: f64, t16123: f64, t16223: f64, t16230: f64, t1659: f64, t25553: f64, t27526: f64, t27527: f64, t375: f64, t7111: f64, t93658: f64, t93667: f64, t93752: f64, t93799: f64, t93801: f64) -> f64 {
    let t100289 = 0.6351706387862183255e-3_f64 * t7132 * t15775;
    let t100302 = t3299 * t100054;
    let t100310 = 0.14481890564325777821e-1_f64 * t1659 * t25553 * t375 + t100289 - 0.57165357490759649296e-3_f64 * t93752 * t15697 - 0.57165357490759649296e-3_f64 * t93752 * t15586 + 0.95275595817932748826e-3_f64 * t100030 * t16223 - 0.30488190661738479624e-2_f64 * t93799 - 0.19055119163586549765e-3_f64 * t93801 + 0.85748036236139473944e-3_f64 * t93667 * t16027 + t7111 * t16123 / 288.0_f64 + 0.11433071498151929859e-2_f64 * t100302 * t16230 + t27526 * t27527 * t15158 / 48.0_f64 - 0.17149607247227894789e-2_f64 * t93658 * t15611;
    t100310
}
