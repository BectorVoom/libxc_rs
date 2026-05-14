//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 652/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk652<F: Float>(t6325: F, t6547: F, t6464: F, t1672: F, t1898: F, t1836: F, t6790: F, t1853: F, t6488: F, t1965: F, t1968: F, t1959: F, t1962: F, t546: F, t6601: F, t1832: F, t6196: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7378 = 0.8311297508363181 * t6325;
    let t7379 = 0.6944444444444444 * t6547;
    let t7384 = 0.2770432502787727 * t6464;
    let t7395 = t1898 * t1672;
    let t7400 = 7.108175748183851 * t1836 * t6790;
    let t7402 = 1.6183441301295518 * t1853 * t6488;
    let t7411 = t1965 * t1672;
    let t7413 = t1968 * t1672;
    let t7415 = t1959 * t1672;
    let t7418 = t1962 * t1672;
    let t7421 = 1.0788960867530346 * t546 * t6601;
    let t7422 = t1832 * t6196;
    (t7378, t7379, t7384, t7395, t7400, t7402, t7411, t7413, t7415, t7418, t7421, t7422)
}
