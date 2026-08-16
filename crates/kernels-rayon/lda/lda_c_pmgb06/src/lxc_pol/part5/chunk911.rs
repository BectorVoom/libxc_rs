//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 911/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk911(t2148: f64, t3760: f64, t3705: f64, t8846: f64, t8850: f64, t26: f64, t5939: f64, t1295: f64, t2236: f64, t247: f64, t28: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11178 = t2148 * t3760;
    let t11180 = t2148 * t3705;
    let t11183 = 144.0_f64 * t8846;
    let t11184 = 8.0_f64 * t8850;
    let t11200 = t5939 * t26;
    let t11206 = t2236 * t1295;
    let t11227 = t769 * t28 * t247;
    (t11178, t11180, t11183, t11184, t11200, t11206, t11227)
}
