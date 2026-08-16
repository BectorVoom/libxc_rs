//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 848/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk848(t2209: f64, t342: f64, t1227: f64, t769: f64, t1282: f64, t34: f64, t1234: f64, t2247: f64, t2248: f64, t3505: f64, t3517: f64, t3525: f64, t3644: f64, t3646: f64, t3654: f64, t5820: f64, t5821: f64, t5825: f64, t5826: f64) -> (f64, f64, f64, f64, f64) {
    let t5866 = t2209 * t342;
    let t5870 = t769 * t1227;
    let t5874 = t34 * t1282;
    let t5875 = t769 * t1234;
    let t5879 = -t3505 - 1.532671111111111_f64 * t3644 + 0.5747516666666667_f64 * t3646 - 1.724255_f64 * t3654 - t3517 + t3525 + t5820 + t5821 - t5825 - t5826 + 10.34553_f64 * t2247 * t2248 * t5866 + 5.172765_f64 * t2247 * t2248 * t5870 - 20.69106_f64 * t2247 * t5874 * t5875;
    (t5866, t5870, t5874, t5875, t5879)
}
