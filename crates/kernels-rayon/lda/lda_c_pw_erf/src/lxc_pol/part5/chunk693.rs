//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 693/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk693(t2526: f64, t581: f64, t549: f64, t1466: f64, t1318: f64, t1401: f64, t593: f64, t571: f64, t2442: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6188 = t581 * t2526;
    let t6189 = t6188 * t549;
    let t6190 = t1466 * t6189;
    let t6192 = 4.0_f64 / 15.0_f64 * t1318 * t6190;
    let t6193 = t1401 * t2526;
    let t6194 = t6193 * t593;
    let t6195 = t1466 * t6194;
    let t6197 = 4.0_f64 / 15.0_f64 * t571 * t6195;
    let t6198 = t2442 * t518;
    (t6188, t6189, t6190, t6192, t6193, t6194, t6195, t6197, t6198)
}
