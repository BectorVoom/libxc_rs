//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 859/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk859<F: Float>(t2414: F, t301: F, t413: F, t297: F, t113: F, t6716: F, t2712: F, t365: F, t350: F, t2715: F, t2696: F, t348: F, t2699: F, t5980: F, t64: F, t35: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6957 = t2414 * t413 * t301;
    let t6958 = t297 * t6957;
    let t6961 = t6716 * t113 * t301;
    let t6967 = t365 * t2712;
    let t6968 = t6967 * t350;
    let t6970 = t365 * t2715;
    let t6971 = t6970 * t350;
    let t6973 = t348 * t2696;
    let t6974 = t6973 * t350;
    let t6975 = 0.9743416666666667 * t6974;
    let t6976 = t348 * t2699;
    let t6977 = t6976 * t350;
    let t6978 = 0.48717083333333333 * t6977;
    let t6979 = t64 * t5980;
    let t6980 = t35 * t6979;
    (t6957, t6958, t6961, t6967, t6968, t6970, t6971, t6973, t6975, t6976, t6978, t6979, t6980)
}
