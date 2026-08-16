//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 952/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk952(t14543: f64, t2174: f64, t415: f64, t5522: f64, t5891: f64, t707: f64, t5895: f64, t1770: f64, t419: f64, t4238: f64, t794: f64, t2257: f64, t4042: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14544 = 0.1890324433388467_f64 * t14543;
    let t14545 = t2174 * t415;
    let t14549 = t5522 * t415;
    let t14550 = 0.1890324433388467_f64 * t14549;
    let t14569 = t707 * t5891;
    let t14570 = 0.11974234010254609_f64 * t14569;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    let t14601 = t2257 * t4042;
    (t14544, t14545, t14550, t14570, t14571, t14575, t14601)
}
