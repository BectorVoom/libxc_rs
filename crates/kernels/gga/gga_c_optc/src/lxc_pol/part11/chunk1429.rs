//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1429/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1429<F: Float>(t5351: F, t5430: F, t17356: F, t4477: F, t17348: F, t1162: F, t1179: F, t17720: F, t17864: F, t18034: F, t18037: F, t3092: F, t3234: F, t3235: F, t3244: F, t3245: F, t4444: F, t4450: F, t58865: F, t59448: F, t59452: F, t59468: F, t59474: F, t59532: F, t59536: F, t59558: F, t914: F) -> (F, F, F, F, F) {
    let t59667 = t5351 * t5351;
    let t59674 = t5430 * t5430;
    let t59711 = t17356 * t4477;
    let t59715 = t17348 * t4477;
    let t59719 = F::new(0.11590881986385010473e0) * t1162 * t914 * t59536 + F::new(0.10747883617784362088e1) * t4444 * t17864 + F::new(0.30228422675018518373e0) * t1179 * t59558 - F::new(0.96161391294453420219e0) * t4450 * t18034 - F::new(0.10818156520626009775e1) * t1162 * t914 * t59468 - F::new(0.30909018630360027928e0) * t4450 * t18037 - F::new(0.69545291918310062836e0) * t1162 * t914 * t59474 - F::new(0.17386322979577515709e0) * t1162 * t914 * t59532 + F::new(0.82101888746963877062e-1) * t1179 * t59452 - F::new(0.10747883617784362088e0) * t4444 * t17720 + F::new(0.50380704458364197288e-2) * t1179 * t59448 - F::new(0.23181763972770020945e0) * t1162 * t914 * t3092 * t58865 + F::new(0.15146801702008125515e1) * t3244 * t3245 * t59711 + F::new(0.93568771831764348721e2) * t3234 * t3235 * t59715;
    (t59667, t59674, t59711, t59715, t59719)
}
