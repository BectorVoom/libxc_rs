//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 429/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk429(t1326: f64, t2006: f64, t1325: f64, t593: f64, t816: f64) -> (f64, f64, f64) {
    let t2007 = t1326 * t2006;
    let t2009 = 8.0_f64 / 45.0_f64 * t1325 * t2007;
    let t2010 = t816 * t593;
    (t2007, t2009, t2010)
}
