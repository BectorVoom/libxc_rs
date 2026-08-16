//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 871/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk871(t1124: f64, t174: f64, t318: f64, t335: f64, t1022: f64, t1010: f64, t386: f64, t400: f64, t2946: f64, t8171: f64, t1059: f64, t2942: f64) -> (f64, f64, f64, f64, f64) {
    let t8427 = 0.22161481481481482_f64 * t174 * t1124 * t318 * t335;
    let t8428 = t1022 * t1022;
    let t8432 = 3.5089340384731225_f64 * t400 * t1010 * t8428 * t386;
    let t8437 = 14.03573615389249_f64 * t400 * t2946 * t8171 * t386;
    let t8438 = t1059 * t2942;
    (t8427, t8428, t8432, t8437, t8438)
}
