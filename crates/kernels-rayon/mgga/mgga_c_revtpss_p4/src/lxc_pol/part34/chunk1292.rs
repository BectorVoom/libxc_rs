//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1292/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1292(t100272: f64, t100329: f64, t100343: f64, t100365: f64, t107101: f64, t107107: f64, t107140: f64, t107154: f64, t107169: f64, t107188: f64, t1972: f64, t23499: f64, t23869: f64, t23874: f64, t23878: f64, t23913: f64, t23917: f64, t23960: f64, t23966: f64, t23980: f64, t24017: f64, t25517: f64, t27526: f64, t27531: f64, t27536: f64, t375: f64, t7111: f64, t7132: f64) -> f64 {
    let t113667 = -t107101 / 144.0_f64 + 0.42874018118069736972e-3_f64 * t23960 * t1972 * t375 + 0.85748036236139473944e-3_f64 * t107107 - 0.28582678745379824648e-3_f64 * t100272 + t27526 * t27531 * t23499 / 72.0_f64 - 0.85748036236139473944e-3_f64 * t107140 + t107154 / 288.0_f64 + 0.1270341277572436651e-2_f64 * t7132 * t23980 + 0.25724410870841842183e-2_f64 * t27536 * t23966 + 0.28582678745379824648e-3_f64 * t100329 - 0.19055119163586549765e-3_f64 * t100343 + t7111 * t24017 / 48.0_f64 + t7111 * t23869 / 288.0_f64 + 7.0_f64 / 648.0_f64 * t7111 * t23874 + t107169 / 216.0_f64 - t7111 * t23878 / 36.0_f64 + 0.85748036236139473944e-3_f64 * t25517 * t23913 + 0.14291339372689912324e-2_f64 * t25517 * t23917 - 0.17149607247227894789e-2_f64 * t107188 - t100365 / 432.0_f64;
    t113667
}
