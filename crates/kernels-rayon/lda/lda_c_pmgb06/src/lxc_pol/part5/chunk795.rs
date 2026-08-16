//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 795/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk795(t2777: f64, t2780: f64, t5615: f64, t5622: f64, t5625: f64, t5627: f64, t5698: f64, t5702: f64, t5712: f64, t7153: f64, t7167: f64, t7176: f64) -> f64 {
    let t7441 = -0.09451622166942335_f64 * t5698 + 0.1890324433388467_f64 * t5702 - 0.07184540406152766_f64 * t5712 - 0.1890324433388467_f64 * t5627 + 0.01975389032890948_f64 * t5615 - 0.01185233419734569_f64 * t5622 - 0.0014862827083471494_f64 * t5625 + 0.02694202652307287_f64 * t7167 - 0.09451622166942335_f64 * t7176 + 0.09451622166942335_f64 * t7153 + t2777 - t2780;
    t7441
}
