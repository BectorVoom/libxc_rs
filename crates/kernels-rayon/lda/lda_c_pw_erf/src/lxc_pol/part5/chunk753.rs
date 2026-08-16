//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 753/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk753(t2072: f64, t6875: f64, t5211: f64, t813: f64, t247: f64, t6039: f64, t251: f64, t2462: f64, t652: f64, t256: f64, t19: f64, t2363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6877 = 8.0_f64 / 15.0_f64 * t6875 * t2072;
    let t6879 = 8.0_f64 / 15.0_f64 * t5211 * t813;
    let t6880 = t6039 * t247;
    let t6881 = t6880 * t251;
    let t6884 = t2462 * t652;
    let t6885 = t6884 * t256;
    let t6887 = t2363 * t19;
    (t6877, t6879, t6880, t6881, t6884, t6885, t6887)
}
