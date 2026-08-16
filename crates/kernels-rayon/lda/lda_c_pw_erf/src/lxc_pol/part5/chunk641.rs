//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 641/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk641(t1476: f64, t2146: f64, t213: f64, t473: f64, t34: f64, t581: f64, t1124: f64, t573: f64, t2152: f64, t571: f64, t1446: f64, t2143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4891 = 16.0_f64 / 135.0_f64 * t2146 * t1476;
    let t4892 = t473 * t213;
    let t4893 = t581 * t34;
    let t4900 = t1124 * t573;
    let t4901 = t4900 * t2152;
    let t4902 = t571 * t4901;
    let t4905 = 16.0_f64 / 135.0_f64 * t1446 * t2143;
    (t4891, t4892, t4893, t4900, t4901, t4902, t4905)
}
