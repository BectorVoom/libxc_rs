//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk614(t1122: f64, t4549: f64, t2148: f64, t980: f64, t968: f64, t2142: f64, t273: f64, t698: f64, t959: f64, t3941: f64, t3945: f64, t3948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4550 = t4549 * t1122;
    let t4552 = t2148 * t980;
    let t4554 = t2148 * t968;
    let t4556 = t2142 * t273;
    let t4558 = 1.1696447245269292_f64 * t4556 * t698;
    let t4559 = t2148 * t959;
    let t4568 = 12.0_f64 * t3941;
    let t4569 = 48.0_f64 * t3945;
    let t4570 = 80.0_f64 * t3948;
    (t4550, t4552, t4554, t4556, t4558, t4559, t4568, t4569, t4570)
}
