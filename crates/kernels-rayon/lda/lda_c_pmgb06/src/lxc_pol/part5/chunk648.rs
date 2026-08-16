//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 648/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk648(t1798: f64, t81: f64, t199: f64, t122: f64, t1669: f64, t886: f64, t107: f64, t1180: f64, t902: f64, t1795: f64, t566: f64, t1329: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5522 = t81 * t1798;
    let t5524 = 0.1675256410710088_f64 * t5522 * t199;
    let t5526 = t122 * t1669 * t886;
    let t5529 = t107 * t1180 * t902;
    let t5542 = 0.1675256410710088_f64 * t1795 * t566;
    let t5551 = 0.1675256410710088_f64 * t1329 * t868;
    (t5522, t5524, t5526, t5529, t5542, t5551)
}
