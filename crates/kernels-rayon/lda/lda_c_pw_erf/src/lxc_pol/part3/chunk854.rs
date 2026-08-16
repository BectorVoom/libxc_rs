//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk854(t2702: f64, t2708: f64, t2711: f64, t2738: f64, t2747: f64, t2751: f64, t2754: f64, t2758: f64, t2760: f64, t2943: f64, t2949: f64, t2988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8099 = 1.9263778438055648_f64 * t2702;
    let t8101 = 0.1301229705933783_f64 * t2708;
    let t8102 = 0.08674864706225219_f64 * t2711;
    let t8103 = 2.339289358982082_f64 * t2738;
    let t8106 = 3.436685857643691_f64 * t2747;
    let t8107 = 0.2849333333333333_f64 * t2751;
    let t8108 = 0.2137_f64 * t2754;
    let t8109 = 0.4274_f64 * t2758;
    let t8110 = 48.0_f64 * t2760;
    let t8113 = 14.03573615389249_f64 * t2943;
    let t8114 = 415.5781415850207_f64 * t2949;
    let t8118 = 4101.558808403118_f64 * t2988;
    (t8099, t8101, t8102, t8103, t8106, t8107, t8108, t8109, t8110, t8113, t8114, t8118)
}
