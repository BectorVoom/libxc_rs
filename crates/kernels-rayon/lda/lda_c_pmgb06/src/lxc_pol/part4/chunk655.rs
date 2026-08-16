//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 655/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk655(t147: f64, t740: f64, t146: f64, t164: f64, t2899: f64, t1400: f64, t187: f64, t186: f64, t395: f64, t184: f64, t1403: f64, t1410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3365 = t740 * t147;
    let t3368 = 0.02962962962962963_f64 * t146 * t3365 * t164;
    let t3380 = 0.11197407407407407_f64 * t2899;
    let t3387 = t1400 * t187;
    let t3389 = t395 * t186;
    let t3391 = 0.0011033703703703704_f64 * t184 * t3389;
    let t3392 = t1403 * t187;
    let t3395 = 4.0_f64 * t1410 * t187;
    (t3365, t3368, t3380, t3387, t3389, t3391, t3392, t3395)
}
