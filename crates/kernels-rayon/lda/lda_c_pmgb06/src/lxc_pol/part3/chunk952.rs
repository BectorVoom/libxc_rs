//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 952/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk952(t11155: f64, t2151: f64, t3734: f64, t11140: f64, t11141: f64, t11142: f64, t11145: f64, t11147: f64, t11150: f64, t11152: f64, t8798: f64, t8814: f64, t8822: f64, t8824: f64, t8826: f64, t8830: f64, t8834: f64) -> f64 {
    let t11156 = 0.0007324578922402618_f64 * t11155;
    let t11157 = t2151 * t3734;
    let t11159 = t11140 - t8798 - t11141 + 3.5089341735807875_f64 * t11142 - 0.0005493434191801964_f64 * t11145 - 51.94757731704439_f64 * t11147 - t11150 - t11152 + t8814 + t8822 - 1.7544670867903938_f64 * t8824 - 51.94757731704439_f64 * t8826 + t8830 - t8834 + t11156 - 0.0005696894717424259_f64 * t11157;
    t11159
}
