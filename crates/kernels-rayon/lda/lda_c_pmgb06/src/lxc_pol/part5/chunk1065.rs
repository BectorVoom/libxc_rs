//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1065/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1065(t486: f64, t7618: f64, t14348: f64, t14350: f64, t14357: f64, t14359: f64, t19736: f64, t19738: f64, t19739: f64, t19740: f64, t19741: f64, t19742: f64, t19746: f64) -> (f64, f64) {
    let t19748 = t486 * t7618 / 30.0_f64;
    let t19751 = -t19736 - t19738 - t19739 - t19740 + t19741 + t19742 + t19746 + t19748 + t14348 + 0.10063568466999305_f64 * t14350 + t14357 + 0.9738937226128359_f64 * t14359;
    (t19748, t19751)
}
