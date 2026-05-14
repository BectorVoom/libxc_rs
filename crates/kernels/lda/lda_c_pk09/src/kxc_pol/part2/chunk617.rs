//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 617/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk617<F: Float>(t421: F, t4801: F, t1690: F, t4804: F, t1151: F, t1697: F, t1706: F, t4814: F, t4813: F, t1161: F, t1156: F, t418: F, t1701: F, t1705: F, t4830: F, t1696: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6347 = 1.28 * t4801 * t421;
    let t6349 = 2.56 * t4804 * t1690;
    let t6350 = t1151 * t1697;
    let t6352 = t1151 * t1706;
    let t6354 = t421 * t4814;
    let t6356 = 2.56 * t4813 * t6354;
    let t6357 = t1697 * t1161;
    let t6358 = t1156 * t6357;
    let t6360 = t1156 * t418;
    let t6361 = t1701 * t1161;
    let t6362 = t6361 * t1705;
    let t6363 = t6360 * t6362;
    let t6365 = t421 * t4830;
    let t6367 = 1.28 * t1156 * t6365;
    let t6376 = t1696 * t1701;
    (t6347, t6349, t6350, t6352, t6356, t6358, t6360, t6363, t6367, t6376)
}
