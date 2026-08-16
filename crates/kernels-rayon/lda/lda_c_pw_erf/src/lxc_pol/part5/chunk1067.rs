//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1067/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1067(t50: f64, t17673: f64, t1789: f64, t20019: f64, t20027: f64, t352: f64, t4367: f64, t4370: f64, t52: f64, t5997: f64, t6005: f64, t7365: f64, t7370: f64, t8334: f64, t943: f64, t950: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t20031 = piecewise3(t51, 0.0_f64, 40.0_f64 / 81.0_f64 * t8334 * t7365 * t352 + 16.0_f64 / 9.0_f64 * t5997 * t943 - 8.0_f64 / 9.0_f64 * t4367 * t17673 - 8.0_f64 / 3.0_f64 * t4370 * t20019 + 4.0_f64 / 3.0_f64 * t1789 * t6005 + 4.0_f64 / 9.0_f64 * t950 * t7370 * t352 + 4.0_f64 / 3.0_f64 * t52 * t20027);
    t20031
}
