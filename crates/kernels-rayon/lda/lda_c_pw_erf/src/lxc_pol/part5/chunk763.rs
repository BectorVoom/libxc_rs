//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 763/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk763(t577: f64, t7007: f64, t2193: f64, t4763: f64, t5340: f64, t5343: f64, t2498: f64, t514: f64, t185: f64, t2076: f64, t2137: f64, t5365: f64, t5373: f64, t5380: f64, t5399: f64, t5411: f64, t5423: f64, t5871: f64, t5872: f64, t5874: f64, t7001: f64, t7006: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7009 = 8.0_f64 / 45.0_f64 * t7007 * t577;
    let t7011 = 8.0_f64 / 15.0_f64 * t4763 * t2193;
    let t7014 = 16.0_f64 / 405.0_f64 * t5340;
    let t7015 = 16.0_f64 / 405.0_f64 * t5343;
    let t7016 = t514 * t2498;
    let t7017 = t185 * t7016;
    let t7018 = 4.0_f64 / 45.0_f64 * t7017;
    let t7019 = t2076 * t2137;
    let t7020 = 16.0_f64 / 45.0_f64 * t7019;
    let t7021 = -t7001 + t7006 + t7009 - t7011 + t5871 + 8.0_f64 / 9.0_f64 * t5872 - 4.0_f64 / 27.0_f64 * t5874 - t7014 - t7015 - t5365 + t5373 - t5380 + t5399 + t5411 - t5423 - t7018 + t7020;
    (t7009, t7011, t7014, t7015, t7016, t7017, t7018, t7019, t7020, t7021)
}
