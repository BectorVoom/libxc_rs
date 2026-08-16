//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 884/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk884(t1423: f64, t2493: f64, t1447: f64, t2489: f64, t2481: f64, t2485: f64, t5220: f64, t806: f64, t2477: f64, t5194: f64, t835: f64, t2462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6307 = t1423 * t2493;
    let t6308 = 4.0_f64 / 135.0_f64 * t6307;
    let t6309 = t1447 * t2489;
    let t6310 = 4.0_f64 / 135.0_f64 * t6309;
    let t6311 = t1423 * t2481;
    let t6312 = 2.0_f64 / 135.0_f64 * t6311;
    let t6313 = t1423 * t2485;
    let t6314 = 2.0_f64 / 81.0_f64 * t6313;
    let t6315 = t5220 * t806;
    let t6316 = 4.0_f64 / 135.0_f64 * t6315;
    let t6317 = t1423 * t2477;
    let t6318 = 4.0_f64 / 135.0_f64 * t6317;
    let t6319 = t5194 * t835;
    let t6320 = 4.0_f64 / 135.0_f64 * t6319;
    let t6321 = t1447 * t2462;
    (t6308, t6310, t6312, t6314, t6316, t6318, t6320, t6321)
}
