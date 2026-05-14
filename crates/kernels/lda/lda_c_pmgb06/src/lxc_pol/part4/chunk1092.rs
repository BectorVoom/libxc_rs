//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1092/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1092<F: Float>(t12: F, t2648: F, t3031: F, t1594: F, t1966: F, t439: F, t8193: F, t1083: F, t12294: F, t2389: F, t247: F, t337: F, t395: F, t5974: F, t6678: F, t764: F, t131: F, t178: F, t44: F, zeta_threshold: F) -> (F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t16317 = t3031 * t2648;
    let t16321 = t439 * t1966 * t16317 * t1594 / 5.0;
    let t16322 = 16.0 * t8193;
    let t16332 = piecewise3(t13, 0.0, 2.0 * t1083 * t2389 + 24.0 * t247 * t6678 + 4.0 * t337 * t5974 - 8.0 * t395 * t764 - t12294 + t16322);
    let t16336 = t16332 * t44 * t131 * t178 / 30.0;
    (t16321, t16322, t16336)
}
