//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 612/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk612(t1108: f64, t898: f64, t2142: f64, t27: f64, t693: f64, t1112: f64, t2151: f64, t2160: f64, t643: f64, t2158: f64, t638: f64, t3765: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4527 = t1108 * t898;
    let t4529 = t2142 * t27;
    let t4531 = 0.0003662289461201309_f64 * t4529 * t693;
    let t4532 = t2151 * t1112;
    let t4534 = t643 * t2160;
    let t4537 = 8.0_f64 * t638 * t2158;
    let t4544 = 4.0_f64 * t3765;
    (t4527, t4529, t4531, t4532, t4534, t4537, t4544)
}
