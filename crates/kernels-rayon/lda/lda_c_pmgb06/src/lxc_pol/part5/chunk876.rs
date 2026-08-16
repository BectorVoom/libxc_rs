//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 876/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk876(t248: f64, t258: f64, t8887: f64, t8925: f64, t8990: f64, t9033: f64, t107: f64, t2786: f64, t701: f64, t290: f64, t8170: f64, t139: f64, t1767: f64) -> (f64, f64, f64, f64) {
    let t9037 = t248 * t258 * (t8887 + t8925 + t8990 + t9033);
    let t9066 = t107 * t2786 * t701;
    let t9070 = 19.1926369973667_f64 * t107 * t8170 * t290;
    let t9175 = t1767 * t139;
    (t9037, t9066, t9070, t9175)
}
