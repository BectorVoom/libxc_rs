//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 436/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk436(t1527: f64, t565: f64, t835: f64, t331: f64, t830: f64, t1371: f64, t1944: f64, t1949: f64, t589: f64, t1210: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2044 = 4.0_f64 / 45.0_f64 * t1527;
    let t2046 = 2.0_f64 / 15.0_f64 * t565 * t835;
    let t2053 = t331 * t830;
    let t2055 = t1371 * t1944;
    let t2058 = t589 * t1949;
    let t2061 = t21 * t1210;
    (t2044, t2046, t2053, t2055, t2058, t2061)
}
