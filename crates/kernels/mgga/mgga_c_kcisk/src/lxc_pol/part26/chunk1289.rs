//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1289/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1289<F: Float>(t21559: F, t964: F, t33921: F, t25: F, t33924: F, t9536: F, t32422: F, t9851: F, t32401: F, t32433: F, t33873: F, t115026: F, t9516: F, t113853: F, t113857: F, t1310: F, t13893: F, t539: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115162 = t964 * t21559;
    let t115169 = t964 * t33921;
    let t115179 = 0.15432098765432098765e-2 * t9536 * t25 * t33921 * t33924;
    let t115213 = 0.34722222222222222222e-2 * t9851 * t32422;
    let t115215 = 0.34722222222222222222e-2 * t9851 * t32401;
    let t115240 = t32433 * t33873;
    let t115247 = t9516 * t115026;
    let t115251 = 0.15476481481481481481e-2 * t113853;
    let t115253 = 0.15476481481481481481e-2 * t113857;
    let t115283 = t1310 * t13893 * t539;
    (t115162, t115169, t115179, t115213, t115215, t115240, t115247, t115251, t115253, t115283)
}
