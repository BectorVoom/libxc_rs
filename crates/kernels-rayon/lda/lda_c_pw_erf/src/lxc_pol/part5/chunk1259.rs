//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1259/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1259(t22593: f64, t2337: f64, t833: f64, t352: f64, t4506: f64, t4522: f64, t20823: f64, t3974: f64, t5160: f64, t3976: f64, t549: f64, t593: f64, t6728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22594 = 8.0_f64 / 15.0_f64 * t22593;
    let t22595 = t2337 * t833;
    let t22596 = t22595 * t352;
    let t22599 = 4.0_f64 / 9.0_f64 * t4506 * t4522 * t22596;
    let t22602 = 16.0_f64 / 15.0_f64 * t3974 * t5160 * t20823;
    let t22606 = 8.0_f64 / 15.0_f64 * t3974 * t3976 * t22595 * t549;
    let t22610 = 8.0_f64 / 15.0_f64 * t4506 * t6728 * t22595 * t593;
    (t22594, t22596, t22599, t22602, t22606, t22610)
}
