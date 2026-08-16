//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1001/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1001(t1435: f64, t2475: f64, t2497: f64, t1342: f64, t9836: f64, t1307: f64, t10186: f64, t1476: f64, t1215: f64, t2689: f64, t2629: f64, t2667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10823 = t2475 * t1435;
    let t10825 = t2497 * t1435;
    let t10827 = t1342 * t9836;
    let t10829 = t1307 * t9836;
    let t10841 = t1476 * t10186;
    let t10843 = t2689 * t1215;
    let t10846 = t2629 * t1435;
    let t10848 = t2667 * t1435;
    (t10823, t10825, t10827, t10829, t10841, t10843, t10846, t10848)
}
