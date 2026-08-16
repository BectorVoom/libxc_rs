//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 777/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk777(t2610: f64, t411: f64, t2594: f64, t1832: f64, t756: f64, t133: f64, t7145: f64, t7142: f64, t1870: f64, t1871: f64, t3322: f64, t5609: f64, t5651: f64, t5660: f64, t5663: f64, t7128: f64, t7155: f64, t7160: f64, t7163: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7191 = t2610 * t411;
    let t7195 = t2594 * t411;
    let t7199 = t756 * t1832;
    let t7203 = t133 * t7145;
    let t7205 = t133 * t7142;
    let t7210 = -t7128 + 5.172765_f64 * t1870 * t1871 * t7191 - 20.69106_f64 * t1870 * t5651 * t7195 + 10.34553_f64 * t1870 * t1871 * t7199 + 0.5747516666666667_f64 * t7203 - 1.724255_f64 * t7205 - 1.724255_f64 * t133 * t7155 - 1.532671111111111_f64 * t5660 + t5663 - t5609 - t7160 + t7163 - t3322;
    (t7191, t7195, t7199, t7203, t7205, t7210)
}
