//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1099/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1099(t1310: f64, t5334: f64, t1472: f64, t4770: f64, t3802: f64, t519: f64, t5243: f64, t10463: f64, t1972: f64, t12829: f64, t12832: f64, t12836: f64, t12839: f64, t12842: f64, t12844: f64, t12846: f64, t12848: f64, t12853: f64) -> (f64, f64, f64, f64, f64) {
    let t12855 = 8.0_f64 / 15.0_f64 * t5334 * t1310;
    let t12857 = 8.0_f64 / 15.0_f64 * t1472 * t4770;
    let t12859 = t519 * t3802 * t5243;
    let t12860 = 8.0_f64 / 45.0_f64 * t12859;
    let t12862 = t519 * t10463 * t1972;
    let t12863 = 16.0_f64 / 135.0_f64 * t12862;
    let t12864 = -t12829 + t12832 - t12836 + t12839 + t12842 + t12844 - t12846 - t12848 - t12853 - t12855 - t12857 - t12860 + t12863;
    (t12855, t12857, t12860, t12863, t12864)
}
