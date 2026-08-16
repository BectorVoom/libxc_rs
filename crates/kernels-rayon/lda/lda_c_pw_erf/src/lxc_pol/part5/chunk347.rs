//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 347/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk347(t1260: f64, t190: f64, t212: f64, t331: f64, t590: f64, t204: f64, t205: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t1366 = 0.011111111111111112_f64 * t190 * t1260 * t212;
    let t1367 = t331 * t590;
    let t1370 = 1.0_f64 / t205 / t204;
    let t1371 = t191 * t1370;
    (t1366, t1367, t1370, t1371)
}
