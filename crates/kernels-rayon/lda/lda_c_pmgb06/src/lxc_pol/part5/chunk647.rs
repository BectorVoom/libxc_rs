//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 647/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk647(t1983: f64, t5499: f64, t122: f64, t2116: f64, t569: f64, t107: f64, t2164: f64, t410: f64, t199: f64, t2174: f64, t718: f64, t868: f64) -> (f64, f64, f64, f64, f64) {
    let t5500 = t5499 * t1983;
    let t5514 = 0.039794582218349216_f64 * t122 * t569 * t2116;
    let t5517 = 1.1389037339096726_f64 * t107 * t410 * t2164;
    let t5518 = t2174 * t199;
    let t5520 = t718 * t868;
    (t5500, t5514, t5517, t5518, t5520)
}
