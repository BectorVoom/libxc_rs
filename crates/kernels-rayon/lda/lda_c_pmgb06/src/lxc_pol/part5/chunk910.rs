//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 910/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk910(t4556: f64, t980: f64, t2148: f64, t3711: f64, t959: f64, t3742: f64, t968: f64, t1065: f64, t2142: f64, t248: f64, t3890: f64, t897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11160 = t4556 * t980;
    let t11161 = 3.5089341735807875_f64 * t11160;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11165 = 1.7544670867903938_f64 * t11164;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11169 = 51.94757731704439_f64 * t11168;
    let t11174 = t248 * t2142 * t1065;
    let t11175 = 3.0_f64 * t11174;
    let t11177 = t248 * t897 * t3890;
    (t11161, t11162, t11165, t11166, t11169, t11175, t11177)
}
