//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 913/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk913(t1837: f64, t831: f64, t6612: f64, t6615: f64, t6618: f64, t6620: f64, t6623: f64, t6625: f64, t6628: f64, t6632: f64, t6635: f64, t6641: f64, t6645: f64, t6648: f64, t6650: f64, t6652: f64) -> (f64, f64) {
    let t6654 = t831 * t1837 / 15.0_f64;
    let t6655 = -t6612 - t6615 - t6618 - t6620 - t6623 - t6625 + t6628 + t6632 - t6635 + t6641 + t6645 - t6648 + t6650 + t6652 + t6654;
    (t6654, t6655)
}
