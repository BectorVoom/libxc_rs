//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 883/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk883(t1062: f64, t22: f64, t19: f64, t301: f64, t305: f64, t732: f64, t8359: f64, t1022: f64, t2986: f64, t1012: f64, t2983: f64, t400: f64) -> (f64, f64, f64, f64) {
    let t8363 = 1.0_f64 / t22 / t1062;
    let t8368 = 0.3407285805772476_f64 * t305 / t8359 * t8363 * t301 * t19 * t732;
    let t8370 = t2986 * t1022;
    let t8373 = 6152.338212604677_f64 * t400 * t2983 * t1012 * t8370;
    (t8363, t8368, t8370, t8373)
}
