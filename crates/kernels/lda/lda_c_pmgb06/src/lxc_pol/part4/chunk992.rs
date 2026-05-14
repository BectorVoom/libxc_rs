//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 992/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk992<F: Float>(t1447: F, t5464: F, t5467: F, t5471: F, t5474: F, t5499: F, t1912: F, t3226: F, t4728: F, t4732: F, t5442: F, t1916: F, t5448: F, t1894: F, t3220: F, t1898: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13844 = t1447 * t5464;
    let t13846 = t1447 * t5467;
    let t13848 = t1447 * t5471;
    let t13850 = t5499 * t5474;
    let t13883 = t3226 * t1912;
    let t13885 = t1447 * t4728;
    let t13887 = t1447 * t4732;
    let t13891 = t5499 * t5442;
    let t13893 = t3226 * t1916;
    let t13895 = t1447 * t5448;
    let t13905 = t3220 * t1894;
    let t13907 = t3220 * t1898;
    (t13844, t13846, t13848, t13850, t13883, t13885, t13887, t13891, t13893, t13895, t13905, t13907)
}
