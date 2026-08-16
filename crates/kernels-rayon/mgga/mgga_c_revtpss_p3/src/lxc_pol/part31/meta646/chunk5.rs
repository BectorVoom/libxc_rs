//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2117/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2117(t18394: f64, t7025: f64, t27221: f64, t62403: f64, t18352: f64, t1945: f64, t807: f64, t61639: f64, t99062: f64, t61725: f64, t103329: f64, t103347: f64, t93049: f64, t93067: f64, t93073: f64, t93088: f64, t99100: f64, t99103: f64) -> f64 {
    let t106093 = t7025 * t18394;
    let t106099 = t27221 * t62403;
    let t106102 = t807 * t1945 * t18352;
    let t106104 = t99062 * t61639;
    let t106106 = t27221 * t61725;
    let t106108 = -t106093 / 48.0_f64 - t103329 - 0.11337795902333997111e-1_f64 * t93049 + t99100 - t99103 - 0.45351183609335988444e-1_f64 * t93067 + 0.10841600599314203355e-2_f64 * t93073 - 0.15244095330869239812e-3_f64 * t93088 - t103347 + t106099 / 16.0_f64 - 0.28582678745379824648e-3_f64 * t106102 - t106104 / 4.0_f64 + t106106 / 8.0_f64;
    t106108
}
