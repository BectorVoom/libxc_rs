//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1248/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1248(t4738: f64, t6689: f64, t6693: f64, t17637: f64, t1996: f64, t3965: f64, t18192: f64, t595: f64, t7470: f64, t184: f64, t811: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22385 = t4738 * t6689;
    let t22386 = 16.0_f64 / 15.0_f64 * t22385;
    let t22388 = 8.0_f64 / 5.0_f64 * t4738 * t6693;
    let t22391 = 8.0_f64 / 15.0_f64 * t3965 * t17637 * t1996;
    let t22392 = 32.0_f64 / 45.0_f64 * t18192;
    let t22394 = 4.0_f64 / 5.0_f64 * t7470 * t595;
    let t22396 = t811 * t820 * t184;
    (t22386, t22388, t22391, t22392, t22394, t22396)
}
