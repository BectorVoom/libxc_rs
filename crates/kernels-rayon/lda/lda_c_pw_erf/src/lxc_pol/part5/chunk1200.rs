//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1200/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1200(t519: f64, t7683: f64, t9304: f64, t17718: f64, t17753: f64, t17768: f64, t12314: f64, t6753: f64, t16602: f64, t1949: f64, t4506: f64, t1944: f64, t2526: f64, t4521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21737 = t519 * t9304 * t7683;
    let t21738 = 16.0_f64 / 45.0_f64 * t21737;
    let t21739 = 32.0_f64 / 135.0_f64 * t17718;
    let t21740 = 8.0_f64 / 15.0_f64 * t17753;
    let t21741 = 8.0_f64 / 45.0_f64 * t17768;
    let t21743 = 16.0_f64 / 9.0_f64 * t12314 * t6753;
    let t21746 = 8.0_f64 / 15.0_f64 * t4506 * t16602 * t1949;
    let t21750 = 4.0_f64 / 9.0_f64 * t4506 * t4521 * t2526 * t1944;
    (t21738, t21739, t21740, t21741, t21743, t21746, t21750)
}
