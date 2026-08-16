//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 969/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk969(t1770: f64, t8085: f64, t31: f64, t4001: f64, t122: f64, t302: f64, t1755: f64, t1773: f64, t1759: f64, t1763: f64, t4294: f64, t707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8087 = 0.0012955432484775182_f64 * t8085 * t1770;
    let t8088 = t31 * t4001;
    let t8091 = 0.9106331049773876_f64 * t122 * t8088 * t302;
    let t8092 = t1773 * t1755;
    let t8094 = t1773 * t1759;
    let t8097 = 0.31931290694012293_f64 * t1773 * t1763;
    let t8099 = 0.07982822673503073_f64 * t707 * t4294;
    (t8087, t8088, t8091, t8092, t8094, t8097, t8099)
}
