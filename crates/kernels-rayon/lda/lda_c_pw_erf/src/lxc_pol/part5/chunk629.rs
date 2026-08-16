//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 629/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk629(t2001: f64, t3854: f64, t1318: f64, t1519: f64, t795: f64, t2123: f64, t565: f64, t790: f64, t925: f64, t1968: f64, t325: f64, t1973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4581 = t3854 * t2001;
    let t4583 = 32.0_f64 / 135.0_f64 * t1318 * t4581;
    let t4592 = t795 * t1519;
    let t4595 = 8.0_f64 / 45.0_f64 * t565 * t2123;
    let t4600 = t925 * t790;
    let t4602 = t325 * t1968;
    let t4604 = t325 * t1973;
    (t4581, t4583, t4592, t4595, t4600, t4602, t4604)
}
