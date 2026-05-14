//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 267/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk267<F: Float>(t1222: F, t94: F, t332: F, t225: F, t282: F, t68: F, t10: F, t9: F, t215: F, t599: F, t221: F, t584: F, t596: F, t600: F, t606: F) -> (F, F, F, F, F, F, F, F) {
    let t1223 = t1222 * t94;
    let t1224 = t332 * t1223;
    let t1225 = 7.35994946043302 * t1224;
    let t1226 = t225 * t282;
    let t1227 = t1226 * t68;
    let t1228 = t9 * t10;
    let t1232 = t215 * t599;
    let t1235 = t584 * t221;
    let t1240 = 0.028458728544442837 * t1228 * t584 * t215 - 0.13318739042300334 * t1232 * t596 + 0.004023984722077967 * t600 * t1235 - 0.008569245379942334 * t606 * t1235;
    (t1223, t1224, t1225, t1226, t1227, t1228, t1232, t1240)
}
