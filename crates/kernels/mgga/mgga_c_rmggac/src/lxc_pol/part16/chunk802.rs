//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 802/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk802<F: Float>(t1587: F, t236: F, t3352: F, t615: F, t7230: F, t10044: F, t1982: F, t7428: F, t8365: F, t8562: F, t131: F, t6344: F, t638: F, t639: F, t71: F, t356: F, t9745: F) -> (F, F, F, F, F) {
    let t44906 = t7230 * t3352 * t236 * t1587 * t615;
    let t44909 = t10044 * t7428 * t1982;
    let t44911 = t8365 * t8562;
    let t44916 = t638 * t639 * t71 * t6344 * t131;
    let t44920 = t638 * t639 * t9745 * t356;
    (t44906, t44909, t44911, t44916, t44920)
}
