//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 298/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk298(t1025: f64, t1043: f64, t1041: f64, t109: f64, t675: f64, t273: f64, t978: f64, t682: f64, t964: f64, t957: f64, t963: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1044 = t1025 * t1043;
    let t1046 = 16.081979498692537_f64 * t1041 * t1044;
    let t1050 = t109 * t675;
    let t1054 = t273 * t978;
    let t1055 = t964 * t682;
    let t1058 = t957 * t682;
    let t1061 = t273 * t963;
    let t1062 = t964 * t967;
    (t1044, t1046, t1050, t1054, t1055, t1058, t1061, t1062)
}
