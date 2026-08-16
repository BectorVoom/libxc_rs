//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1190/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1190(t2002: f64, t5226: f64, t5242: f64, t5245: f64, t5248: f64, t6275: f64, t1902: f64, t5187: f64, t5254: f64, t5257: f64, t5261: f64, t1916: f64, t5305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15703 = 8.0_f64 / 45.0_f64 * t2002 * t5226;
    let t15705 = 4.0_f64 / 45.0_f64 * t2002 * t5242;
    let t15707 = 4.0_f64 / 9.0_f64 * t2002 * t5245;
    let t15709 = 16.0_f64 / 45.0_f64 * t6275 * t5248;
    let t15711 = 4.0_f64 / 27.0_f64 * t5187 * t1902;
    let t15713 = 4.0_f64 / 27.0_f64 * t2002 * t5254;
    let t15715 = 2.0_f64 / 27.0_f64 * t2002 * t5257;
    let t15717 = 16.0_f64 / 81.0_f64 * t2002 * t5261;
    let t15719 = 8.0_f64 / 45.0_f64 * t5305 * t1916;
    (t15703, t15705, t15707, t15709, t15711, t15713, t15715, t15717, t15719)
}
