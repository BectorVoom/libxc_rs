//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1137/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1137<F: Float>(t1329: F, t15579: F, t1403: F, t2419: F, t3867: F, t571: F, t1319: F, t16278: F, t1472: F, t6409: F, t1308: F, t1381: F, t2415: F, t16683: F, t16687: F, t16692: F, t16696: F, t16698: F, t16700: F, t16703: F, t16708: F, t16710: F, t16712: F, t16716: F, t16721: F) -> (F, F, F, F, F, F) {
    let t16723 = 16.0 / 45.0 * t15579 * t1329;
    let t16727 = 8.0 / 45.0 * t571 * t3867 * t2419 * t1403;
    let t16730 = 8.0 / 15.0 * t571 * t1319 * t16278;
    let t16732 = 16.0 / 45.0 * t1472 * t6409;
    let t16736 = 8.0 / 45.0 * t571 * t1308 * t2415 * t1381;
    let t16737 = -t16683 + t16687 + t16692 + t16696 - t16698 + t16700 + t16703 - t16708 - t16710 + t16712 - t16716 + t16721 + t16723 + t16727 + t16730 + t16732 + t16736;
    (t16723, t16727, t16730, t16732, t16736, t16737)
}
