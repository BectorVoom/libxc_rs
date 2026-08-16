//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1126/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1126(t1440: f64, t15590: f64, t519: f64, t806: f64, t1446: f64, t7605: f64, t15582: f64, t2158: f64, t20861: f64, t20864: f64, t20868: f64, t20870: f64, t20873: f64, t20876: f64, t20879: f64, t20882: f64, t20885: f64, t20886: f64) -> (f64, f64, f64, f64) {
    let t20890 = 4.0_f64 / 5.0_f64 * t519 * t1440 * t15590 * t806;
    let t20892 = 4.0_f64 / 5.0_f64 * t1446 * t7605;
    let t20894 = 4.0_f64 / 5.0_f64 * t15582 * t2158;
    let t20895 = t20861 - t20864 + t20868 - t20870 - t20873 + t20876 + t20879 + t20882 + t20885 + t20886 + t20890 + t20892 - t20894;
    (t20890, t20892, t20894, t20895)
}
