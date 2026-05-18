//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1151/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1151<F: Float>(t10170: F, t6316: F, t1045: F, t4670: F, t4848: F, t6353: F, t829: F, t3073: F, t6352: F, t10093: F, t10108: F, t1030: F, t13790: F, t18623: F, t19324: F, t19327: F, t19330: F, t19332: F, t19334: F, t19336: F, t19340: F, t19344: F, t19347: F, t19350: F, t19353: F, t19356: F, t19360: F, t19363: F, t305: F, t3061: F, t3158: F) -> F {
    let t19366 = t10170 * t6316;
    let t19367 = t19366 * t1045;
    let t19370 = t4848 * t4670;
    let t19373 = t6353 * t829;
    let t19376 = t3073 * t6352;
    let t19377 = t19376 * t1045;
    let t19380 = -F::new(0.23426533963880895498e-2) * t1030 * t19324 - F::new(0.46853067927761790996e-2) * t305 * t19327 - F::new(0.46853067927761790996e-2) * t19330 + F::new(0.23426533963880895498e-2) * t19332 - F::new(0.14055920378328537299e-1) * t19334 + F::new(0.46853067927761790996e-2) * t19336 - F::new(0.18741227171104716398e-1) * t13790 * t18623 - F::new(0.14055920378328537299e-1) * t10093 * t19340 - F::new(0.93706135855523581992e-2) * t3061 * t19344 + F::new(0.18741227171104716398e-1) * t10108 * t19347 + F::new(0.93706135855523581992e-2) * t1030 * t19350 + F::new(0.18741227171104716398e-1) * t3158 * t19353 + F::new(0.46853067927761790996e-2) * t3061 * t19356 + F::new(0.46853067927761790996e-2) * t1030 * t19360 - F::new(0.14055920378328537299e-1) * t1030 * t19363 - F::new(0.56223681513314149196e-1) * t305 * t19367 + F::new(0.28111840756657074598e-1) * t305 * t19370 + F::new(0.46853067927761790996e-2) * t1030 * t19373 + F::new(0.14055920378328537299e-1) * t305 * t19377;
    t19380
}
