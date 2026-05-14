//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1148/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1148<F: Float>(t13245: F, t13249: F, t13251: F, t10087: F, t10089: F, t1444: F, t6752: F, t13182: F, t176: F, t1821: F, t493: F, t4880: F, t6751: F, t13483: F, t4885: F, t1981: F, t4866: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17262 = 8.0 / 135.0 * t13245;
    let t17263 = 8.0 / 135.0 * t13249;
    let t17264 = 8.0 / 135.0 * t13251;
    let t17265 = t10087 / 135.0;
    let t17266 = 2.0 / 135.0 * t10089;
    let t17268 = 4.0 / 27.0 * t1444 * t6752;
    let t17272 = 4.0 / 27.0 * t493 * t13182 * t176 * t1821;
    let t17275 = 2.0 / 27.0 * t493 * t6751 * t4880;
    let t17276 = t13483 * t176;
    let t17279 = 16.0 / 81.0 * t493 * t17276 * t4885;
    let t17282 = 8.0 / 27.0 * t1981 * t6751 * t4866;
    (t17262, t17263, t17264, t17265, t17266, t17268, t17272, t17275, t17279, t17282)
}
