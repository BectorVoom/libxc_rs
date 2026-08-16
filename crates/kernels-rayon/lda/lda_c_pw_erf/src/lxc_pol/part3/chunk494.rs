//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 494/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk494(t1318: f64, t2002: f64, t504: f64, t784: f64, t348: f64, t1326: f64) -> (f64, f64, f64, f64) {
    let t2004 = 8.0_f64 / 45.0_f64 * t1318 * t2002;
    let t2005 = t784 * t504;
    let t2006 = t2005 * t348;
    let t2007 = t1326 * t2006;
    (t2004, t2005, t2006, t2007)
}
