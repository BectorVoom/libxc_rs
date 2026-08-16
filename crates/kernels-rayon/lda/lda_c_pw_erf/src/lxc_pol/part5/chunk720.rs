//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 720/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk720(t4829: f64, t6442: f64, t519: f64, t1948: f64, t34: f64) -> (f64, f64, f64) {
    let t6443 = t4829 * t6442;
    let t6445 = 32.0_f64 / 45.0_f64 * t519 * t6443;
    let t6446 = t1948 * t34;
    (t6443, t6445, t6446)
}
