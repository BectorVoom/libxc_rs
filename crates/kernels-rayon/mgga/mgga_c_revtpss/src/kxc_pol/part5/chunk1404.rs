//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1404/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1404(t14239: f64, t5741: f64, t6844: f64, t72: f64, t686: f64, t4101: f64, t6874: f64, t10098: f64, t10102: f64, t10109: f64, t10114: f64, t14218: f64, t14221: f64, t14227: f64, t14229: f64, t14233: f64, t14241: f64, t14243: f64, t22005: f64, t5675: f64, t5745: f64) -> f64 {
    let t22329 = t14239 * t5741;
    let t22331 = t6844 * t72;
    let t22332 = t22331 * t686;
    let t22333 = t4101 * t22332;
    let t22335 = t6874 * t72;
    let t22336 = t22335 * t686;
    let t22337 = t4101 * t22336;
    let t22344 = -t14218 - 0.23131639038696784278e-2_f64 * t14221 - 0.13009920719177044025e-1_f64 * t10098 + t10102 + t14227 - t14229 - t14233 - 0.19514881078765566037e-1_f64 * t22329 - 0.9757440539382783019e-2_f64 * t22333 - 0.9757440539382783019e-2_f64 * t22337 - t14241 + 0.26019841438354088051e-1_f64 * t14243 + 0.11565819519348392139e-2_f64 * t10109 + t10114 + 0.39512695097613069591e1_f64 * t5745 * t22005 * t5675;
    t22344
}
