//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 745/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk745<F: Float>(t350: F, t6973: F, t2699: F, t348: F, t5980: F, t64: F, t35: F, t110: F, t2703: F, t360: F, t2707: F, t2695: F, t3615: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6974 = t6973 * t350;
    let t6975 = F::new(0.9743416666666667) * t6974;
    let t6976 = t348 * t2699;
    let t6977 = t6976 * t350;
    let t6978 = F::new(0.48717083333333333) * t6977;
    let t6979 = t64 * t5980;
    let t6980 = t35 * t6979;
    let t6983 = t110 * t2703;
    let t6984 = t360 * t6983;
    let t6986 = t110 * t2707;
    let t6987 = t360 * t6986;
    let t6989 = t3615 * t2695;
    (t6974, t6975, t6976, t6977, t6978, t6979, t6980, t6983, t6984, t6986, t6987, t6989)
}
