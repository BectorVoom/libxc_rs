//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1194/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1194(t17505: f64, t17508: f64, t12984: f64, t12987: f64, t17548: f64, t17550: f64, t6671: f64, t835: f64, t2114: f64, t7799: f64, t1298: f64, t186: f64, t198: f64, t21299: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21675 = 8.0_f64 / 5.0_f64 * t17505;
    let t21676 = 16.0_f64 / 15.0_f64 * t17508;
    let t21677 = 32.0_f64 / 135.0_f64 * t12984;
    let t21678 = 8.0_f64 / 45.0_f64 * t12987;
    let t21680 = 8.0_f64 / 15.0_f64 * t17548;
    let t21681 = 16.0_f64 / 15.0_f64 * t17550;
    let t21683 = 2.0_f64 / 5.0_f64 * t6671 * t835;
    let t21685 = 4.0_f64 / 15.0_f64 * t2114 * t7799;
    let t21687 = 4.0_f64 / 15.0_f64 * t1298 * t7799;
    let t21692 = -4.0_f64 / 15.0_f64 * t493 * t186 * t198 * t21299;
    (t21675, t21676, t21677, t21678, t21680, t21681, t21683, t21685, t21687, t21692)
}
