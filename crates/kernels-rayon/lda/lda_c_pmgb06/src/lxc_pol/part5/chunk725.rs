//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 725/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk725(t44: f64, t6703: f64, t131: f64, t155: f64, t2592: f64, t460: f64, t1928: f64, t802: f64, t2029: f64, t4111: f64, t2802: f64, t4461: f64, t4462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6704 = t6703 * t44;
    let t6705 = t6704 * t131;
    let t6707 = t6705 * t155 / 30.0_f64;
    let t6709 = t2592 * t460 / 30.0_f64;
    let t6710 = t802 * t1928;
    let t6711 = 2.0_f64 / 45.0_f64 * t6710;
    let t6715 = (2e-21_f64 as f64) * t2029 * t4111;
    let t6716 = -t4461 + t4462 + t2802;
    (t6704, t6705, t6707, t6709, t6710, t6711, t6715, t6716)
}
