//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 638/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk638(t1944: f64, t4794: f64, t571: f64, t2113: f64, t518: f64) -> (f64, f64, f64) {
    let t4795 = t4794 * t1944;
    let t4797 = 16.0_f64 / 81.0_f64 * t571 * t4795;
    let t4804 = t2113 * t518;
    (t4795, t4797, t4804)
}
