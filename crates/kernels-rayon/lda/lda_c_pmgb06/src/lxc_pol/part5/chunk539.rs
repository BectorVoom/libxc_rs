//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 539/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk539(t118: f64, t2778: f64, t1179: f64, t55: f64, t1767: f64, t32: f64) -> (f64, f64, f64, f64) {
    let t2779 = t2778 * t118;
    let t2780 = 0.00011865309871651405_f64 * t2779;
    let t2781 = t55 * t1179;
    let t2786 = t32 * t1767;
    (t2779, t2780, t2781, t2786)
}
