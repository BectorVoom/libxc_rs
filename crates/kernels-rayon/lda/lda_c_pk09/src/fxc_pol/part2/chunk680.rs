//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 680/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk680(t421: f64, t4801: f64, t1690: f64, t4804: f64, t1151: f64, t1697: f64, t1706: f64, t4814: f64, t4813: f64, t1161: f64, t1156: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6347 = 1.28_f64 * t4801 * t421;
    let t6349 = 2.56_f64 * t4804 * t1690;
    let t6350 = t1151 * t1697;
    let t6352 = t1151 * t1706;
    let t6354 = t421 * t4814;
    let t6356 = 2.56_f64 * t4813 * t6354;
    let t6357 = t1697 * t1161;
    let t6358 = t1156 * t6357;
    let t6360 = t1156 * t418;
    (t6347, t6349, t6350, t6352, t6356, t6358, t6360)
}
