//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 750/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk750<F: Float>(t1491: F, t831: F, t1512: F, t815: F, t1831: F, t529: F, t1380: F, t1981: F, t1444: F, t1916: F, t1450: F, t176: F, t1826: F, t493: F, t1915: F, t4847: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5438 = t831 * t1491 / 30.0;
    let t5440 = t1512 * t815 / 30.0;
    let t5441 = t1831 * t529;
    let t5442 = t1380 * t5441;
    let t5444 = 4.0 / 45.0 * t1981 * t5442;
    let t5446 = 4.0 / 45.0 * t1444 * t1916;
    let t5447 = t1450 * t176;
    let t5448 = t5447 * t1826;
    let t5450 = 4.0 / 45.0 * t493 * t5448;
    let t5451 = t1915 * t4847;
    (t5438, t5440, t5441, t5442, t5444, t5446, t5447, t5448, t5450, t5451)
}
