//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 655/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk655(t1122: f64, t3969: f64, t283: f64, t3933: f64, t3939: f64, t3942: f64, t3944: f64, t3946: f64, t3949: f64, t3954: f64, t3956: f64, t3959: f64, t3962: f64, t3965: f64, t3968: f64) -> (f64, f64) {
    let t3970 = t3969 * t1122;
    let t3972 = 0.0197516734986138_f64 * t3933 * t283 + 36.0_f64 * t3939 + t3942 + t3944 - t3946 + t3949 - t3954 - t3956 - t3959 - t3962 + t3965 + t3968 + 0.03253074390090522_f64 * t3970;
    (t3970, t3972)
}
