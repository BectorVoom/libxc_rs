//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 796/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk796(t125: f64, t1550: f64, t1808: f64, t1881: f64, t2592: f64, t2645: f64, t3203: f64, t411: f64, t4125: f64, t4129: f64, t4132: f64, t4136: f64, t4140: f64, t4144: f64, t456: f64, t5933: f64, t5941: f64, t6097: f64, t7083: f64, t7085: f64, t7214: f64, t7231: f64, t7302: f64, t777: f64) -> f64 {
    let t7305 = 0.19816831758676853_f64 * t3203 + t1881 * t2592 + t777 * t7083 + 12.0_f64 * t1808 * t7085 + t2645 * t1550 + 6.0_f64 * t1808 * t6097 * t411 + t7214 * t456 - 3.64371538634302e-05_f64 * t5933 - 0.0005811348303577384_f64 * t4125 - t4129 + 0.001355981270834723_f64 * t4132 + t4136 - t4140 - t4144 - t5941 + (t7231 + t7302) * t125;
    t7305
}
