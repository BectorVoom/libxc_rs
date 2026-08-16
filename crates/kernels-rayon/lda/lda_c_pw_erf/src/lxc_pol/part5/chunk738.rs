//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk738(t203: f64, t6669: f64, t184: f64, t221: f64, t3910: f64, t3917: f64, t3919: f64, t3923: f64, t3929: f64, t3935: f64, t3938: f64, t3950: f64, t3951: f64, t4595: f64, t6599: f64, t6603: f64, t6605: f64, t6606: f64, t6613: f64, t6633: f64) -> (f64, f64, f64, f64) {
    let t6670 = t203 * t6669;
    let t6671 = t6670 * t184;
    let t6673 = 2.0_f64 / 15.0_f64 * t6671 * t221;
    let t6674 = t6599 + t6603 + t6605 - t6606 - t4595 + 4.0_f64 / 9.0_f64 * t3910 + t3917 + t3919 + t3923 + t3929 + t3935 - t3938 + t3950 / 3.0_f64 + 0.12155555555555556_f64 * t3951 + t6613 + t6633 + t6673;
    (t6670, t6671, t6673, t6674)
}
