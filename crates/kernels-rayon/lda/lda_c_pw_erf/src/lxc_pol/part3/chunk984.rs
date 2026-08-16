//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 984/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk984(t50: f64, t462: f64, t951: f64, t352: f64, t39: f64, t954: f64, t1792: f64, t343: f64, t1789: f64, t2966: f64, t2967: f64, t2973: f64, t34: f64, t4367: f64, t4370: f64, t52: f64, t743: f64, t8334: f64, t9456: f64, t950: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t11437 = t462 * t951;
    let t11445 = t39 * t352;
    let t11448 = t462 * t954;
    let t11456 = 32.0_f64 * t1792 * t343;
    let t11458 = piecewise3(t51, 0.0_f64, 40.0_f64 / 81.0_f64 * t8334 * t743 * t2967 + 16.0_f64 / 9.0_f64 * t2966 * t34 * t11437 - 8.0_f64 / 9.0_f64 * t4367 * t9456 - 8.0_f64 / 3.0_f64 * t950 * t462 * t352 + 8.0_f64 * t4370 * t11445 - 8.0_f64 / 3.0_f64 * t4370 * t11448 + 4.0_f64 / 9.0_f64 * t1789 * t2973 + 16.0_f64 * t52 * t39 - t11456);
    (t11437, t11445, t11448, t11458)
}
