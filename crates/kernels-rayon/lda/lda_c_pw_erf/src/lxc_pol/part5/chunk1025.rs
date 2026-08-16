//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1025(t2127: f64, t5215: f64, t2120: f64, t4564: f64, t185: f64, t514: f64, t6567: f64, t230: f64, t7280: f64, t4729: f64, t795: f64, t5184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17417 = t5215 * t2127;
    let t17423 = t2120 * t4564;
    let t17426 = t185 * t514 * t6567;
    let t17432 = t7280 * t230;
    let t17434 = t795 * t4729;
    let t17436 = t795 * t5184;
    (t17417, t17423, t17426, t17432, t17434, t17436)
}
