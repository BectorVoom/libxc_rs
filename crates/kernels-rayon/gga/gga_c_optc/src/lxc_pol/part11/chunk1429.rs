//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1429/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1429(t5351: f64, t5430: f64, t17356: f64, t4477: f64, t17348: f64, t1162: f64, t1179: f64, t17720: f64, t17864: f64, t18034: f64, t18037: f64, t3092: f64, t3234: f64, t3235: f64, t3244: f64, t3245: f64, t4444: f64, t4450: f64, t58865: f64, t59448: f64, t59452: f64, t59468: f64, t59474: f64, t59532: f64, t59536: f64, t59558: f64, t914: f64) -> (f64, f64, f64, f64, f64) {
    let t59667 = t5351 * t5351;
    let t59674 = t5430 * t5430;
    let t59711 = t17356 * t4477;
    let t59715 = t17348 * t4477;
    let t59719 = 0.11590881986385010473e0_f64 * t1162 * t914 * t59536 + 0.10747883617784362088e1_f64 * t4444 * t17864 + 0.30228422675018518373e0_f64 * t1179 * t59558 - 0.96161391294453420219e0_f64 * t4450 * t18034 - 0.10818156520626009775e1_f64 * t1162 * t914 * t59468 - 0.30909018630360027928e0_f64 * t4450 * t18037 - 0.69545291918310062836e0_f64 * t1162 * t914 * t59474 - 0.17386322979577515709e0_f64 * t1162 * t914 * t59532 + 0.82101888746963877062e-1_f64 * t1179 * t59452 - 0.10747883617784362088e0_f64 * t4444 * t17720 + 0.50380704458364197288e-2_f64 * t1179 * t59448 - 0.23181763972770020945e0_f64 * t1162 * t914 * t3092 * t58865 + 0.15146801702008125515e1_f64 * t3244 * t3245 * t59711 + 0.93568771831764348721e2_f64 * t3234 * t3235 * t59715;
    (t59667, t59674, t59711, t59715, t59719)
}
