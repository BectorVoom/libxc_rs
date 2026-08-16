//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 838/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk838(t2209: f64, t73: f64, t76: f64, t1227: f64, t2181: f64, t1282: f64, t2221: f64, t342: f64, t38: f64, t776: f64, t1234: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5721 = t73 * t2209;
    let t5731 = t76 * t2209;
    let t5737 = t2181 * t1227;
    let t5740 = t1282 * t2209;
    let t5749 = 11.6921_f64 * t38 * t2221 * t342;
    let t5752 = 5.84605_f64 * t38 * t776 * t1227;
    let t5755 = 17.53815_f64 * t38 * t780 * t1234;
    (t5721, t5731, t5737, t5740, t5749, t5752, t5755)
}
