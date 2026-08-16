//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2168/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2168(t30056: f64, t686: f64, t72: f64, t7289: f64, t108280: f64, t108282: f64, t108294: f64, t108296: f64, t108302: f64, t1444: f64, t22433: f64, t25921: f64, t25924: f64, t27837: f64, t27903: f64, t30017: f64, t30020: f64, t30021: f64, t30101: f64, t7279: f64, t7295: f64, t7304: f64, t94662: f64, t97843: f64, t97847: f64) -> (f64, f64) {
    let t108307 = t30056 * t72 * t686;
    let t108308 = t7289 * t108307;
    let t108310 = -0.52041769129231196772e1_f64 * t7295 * t25924 * t30020 * t1444 + 0.17347256376410398924e1_f64 * t27837 * t27903 - 0.12851425765524037203e-1_f64 * t108280 + 0.4336814094102599731e0_f64 * t108282 * t7304 + 0.8673628188205199462e0_f64 * t25921 * t30101 + 0.39029762157531132076e-1_f64 * t97843 + 0.96373646535613327359e-3_f64 * t97847 + 0.19274729307122665471e-1_f64 * t94662 + 0.17347256376410398924e1_f64 * t25921 * t30021 + 0.43368140941025997311e-1_f64 * t108294 - 0.77108554593144223219e-1_f64 * t108296 - 0.39512695097613069591e1_f64 * t7279 * t22433 - 0.54878743191129263322e-2_f64 * t108302 - 0.26020884564615598386e1_f64 * t25921 * t30017 - 0.12851425765524037203e-1_f64 * t108308;
    (t108307, t108310)
}
