//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1977/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1977(t102420: f64, t5722: f64, t28780: f64, t98041: f64, t27899: f64, t28845: f64, t28894: f64, t97802: f64, t98380: f64, t102320: f64, t102324: f64, t102325: f64, t102656: f64, t108244: f64, t14224: f64, t1444: f64, t25921: f64, t25924: f64, t25930: f64, t26304: f64, t27837: f64, t27868: f64, t28806: f64, t30279: f64, t30282: f64, t6895: f64, t7295: f64, t7506: f64, t96374: f64) -> f64 {
    let t109534 = t102420 * t5722;
    let t109536 = t98041 * t28780;
    let t109539 = t27899 * t28845;
    let t109553 = t97802 * t28894;
    let t109555 = t98380 * t28894;
    let t109563 = -t102320 - 0.19514881078765566037e-1_f64 * t109534 + 0.51405703062096148813e-1_f64 * t109536 + t102324 + 0.86736281882051994624e-1_f64 * t102325 + 0.14456046980341999104e-1_f64 * t109539 - 0.26020884564615598386e1_f64 * t25921 * t30279 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t7506 * t6895 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t30282 * t1444 + 0.17347256376410398924e1_f64 * t27837 * t28806 + t96374 - 0.14456046980341999104e-1_f64 * t109553 + 0.25702851531048074406e-1_f64 * t109555 + 0.8673628188205199462e0_f64 * t27868 * t102656 * t14224 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t108244;
    t109563
}
