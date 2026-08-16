//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 600/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk600(t1150: f64, t1155: f64, t1162: f64, t1151: f64, t1175: f64, t1186: f64, t1154: f64, t251: f64, t246: f64, t1161: f64, t272: f64, t1156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4804 = t1150 * t1155;
    let t4806 = 2.56_f64 * t4804 * t1162;
    let t4807 = t1151 * t1175;
    let t4809 = t1151 * t1186;
    let t4812 = 1.0_f64 / t1154 / t251;
    let t4813 = t246 * t4812;
    let t4814 = t1161 * t1161;
    let t4815 = t272 * t4814;
    let t4817 = 2.56_f64 * t4813 * t4815;
    let t4818 = t1175 * t1161;
    let t4819 = t1156 * t4818;
    (t4804, t4806, t4807, t4809, t4813, t4814, t4817, t4819)
}
