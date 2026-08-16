//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 747/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk747(t1414: f64, t518: f64, t529: f64, t764: f64, t337: f64, t5068: f64, t129: f64, t130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5069 = t518 * t1414;
    let t5070 = t764 * t529;
    let t5071 = t5070 * t337;
    let t5072 = t5069 * t5071;
    let t5074 = 4.0_f64 / 45.0_f64 * t5068 * t5072;
    let t5075 = t129 * t130;
    (t5069, t5070, t5071, t5072, t5074, t5075)
}
