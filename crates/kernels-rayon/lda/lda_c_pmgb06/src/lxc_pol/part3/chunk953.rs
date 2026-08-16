//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 953/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk953(t4556: f64, t980: f64, t2148: f64, t3711: f64, t959: f64, t3742: f64, t968: f64, t273: f64, t4515: f64, t698: f64, t1065: f64, t2142: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11160 = t4556 * t980;
    let t11161 = 3.5089341735807875_f64 * t11160;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11165 = 1.7544670867903938_f64 * t11164;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11169 = 51.94757731704439_f64 * t11168;
    let t11171 = t4515 * t273 * t698;
    let t11174 = t248 * t2142 * t1065;
    (t11161, t11162, t11165, t11166, t11169, t11171, t11174)
}
