//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 932/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk932(t107: f64, t2407: f64, t410: f64, t122: f64, t2659: f64, t569: f64, t110: f64, t202: f64, t4063: f64, t4174: f64, t4181: f64, t4185: f64, t5514: f64, t5517: f64, t5518: f64, t5520: f64, t5524: f64, t5526: f64, t5529: f64, t6104: f64, t6913: f64) -> f64 {
    let t6918 = t107 * t410 * t2407;
    let t6922 = t122 * t569 * t2659;
    let t6927 = t5514 + 0.42708890021612717_f64 * t107 * t110 * t6104 - t5517 - 0.3350512821420176_f64 * t5518 - 0.3350512821420176_f64 * t5520 + t5524 - 0.011938374665504766_f64 * t122 * t202 * t6913 - 0.5694518669548363_f64 * t6918 - 0.10611888591559791_f64 * t5526 + 0.019897291109174608_f64 * t6922 + 2.657442045789236_f64 * t5529 - 0.053059442957798957_f64 * t4174 - t4181 + t4185 + 1.328721022894618_f64 * t4063;
    t6927
}
