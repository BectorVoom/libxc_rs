//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 857/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk857(t138: f64, t1711: f64, t1861: f64, t2634: f64, t2642: f64, t3339: f64, t444: f64, t5621: f64, t7168: f64, t774: f64, t7949: f64, t7957: f64, t7960: f64, t7974: f64) -> f64 {
    let t7976 = t7949 * t138 + 6.0_f64 * t1711 * t7960 - 3.0_f64 * t1861 * t2642 + 6.0_f64 * t5621 * t2634 - 6.0_f64 * t3339 * t7957 - t444 * t7974 - 3.0_f64 * t7168 * t774;
    t7976
}
