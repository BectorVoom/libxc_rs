//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 743/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk743(t1954: f64, t352: f64, t4758: f64, t1318: f64, t2075: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t4759 = t1954 * t352;
    let t4760 = t4758 * t4759;
    let t4762 = 16.0_f64 / 45.0_f64 * t1318 * t4760;
    let t4763 = t2075 * t518;
    (t4759, t4760, t4762, t4763)
}
