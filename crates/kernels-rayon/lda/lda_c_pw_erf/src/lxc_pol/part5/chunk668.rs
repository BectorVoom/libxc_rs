//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 668/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk668(t2252: f64, t652: f64, t256: f64, t19: f64, t1904: f64, t644: f64, t647: f64, t1432: f64, t850: f64, t1427: f64, t2260: f64, t1217: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5791 = t2252 * t652;
    let t5793 = 2.0_f64 / 3.0_f64 * t5791 * t256;
    let t5794 = t1904 * t19;
    let t5795 = t5794 * t644;
    let t5797 = 0.12155555555555556_f64 * t5795 * t647;
    let t5798 = t850 * t1432;
    let t5799 = t5798 * t256;
    let t5801 = t2260 * t1427;
    let t5806 = t858 * t1217;
    (t5791, t5793, t5794, t5795, t5797, t5798, t5799, t5801, t5806)
}
