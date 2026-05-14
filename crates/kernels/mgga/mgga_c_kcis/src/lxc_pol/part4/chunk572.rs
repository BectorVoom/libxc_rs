//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 572/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk572<F: Float>(t3135: F, t3165: F, t1030: F, t1083: F, t278: F, t3038: F, t305: F, t3056: F, t3057: F, t3059: F, t3061: F, t3062: F, t3066: F, t3069: F, t3075: F, t3097: F, t339: F, t975: F) -> (F, F) {
    let t3166 = t3135 + t3165;
    let t3168 = t3056 + 0.46853067927761790996e-2 * t3057 + 0.93706135855523581992e-2 * t3059 + 0.46853067927761790996e-2 * t3061 * t3062 + 0.93706135855523581992e-2 * t1030 * t3066 - 0.23426533963880895498e-2 * t1030 * t3069 + 0.14055920378328537299e-1 * t305 * t3075 - 0.46853067927761790996e-2 * t305 * t3097 - t3038 * t339 - 2.0 * t975 * t1083 - t278 * t3166;
    (t3166, t3168)
}
