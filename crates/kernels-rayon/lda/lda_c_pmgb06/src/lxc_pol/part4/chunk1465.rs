//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1465/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1465(t18744: f64, t69: f64, t11515: f64, t11519: f64, t11521: f64, t18656: f64, t18671: f64, t18674: f64, t18677: f64, t18684: f64, t18685: f64, t18688: f64, t18693: f64, t18694: f64, t18696: f64, t18697: f64, t18700: f64) -> f64 {
    let t18837 = t69 * t18744;
    let t18842 = -0.7663355555555555_f64 * t18837 + 1.1495033333333333_f64 * t11515 - 3.065342222222222_f64 * t11519 + 3.5762325925925924_f64 * t11521 + t18656 - t18671 + t18674 + t18677 + t18684 + t18685 + t18688 - t18693 - t18694 - t18696 - t18697 + t18700;
    t18842
}
