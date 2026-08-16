//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 795/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk795<F: Float>(t2777: F, t2780: F, t5615: F, t5622: F, t5625: F, t5627: F, t5698: F, t5702: F, t5712: F, t7153: F, t7167: F, t7176: F) -> F {
    let t7441 = -F::cast_from(0.09451622166942335_f64) * t5698 + F::cast_from(0.1890324433388467_f64) * t5702 - F::cast_from(0.07184540406152766_f64) * t5712 - F::cast_from(0.1890324433388467_f64) * t5627 + F::cast_from(0.01975389032890948_f64) * t5615 - F::cast_from(0.01185233419734569_f64) * t5622 - F::cast_from(0.0014862827083471494_f64) * t5625 + F::cast_from(0.02694202652307287_f64) * t7167 - F::cast_from(0.09451622166942335_f64) * t7176 + F::cast_from(0.09451622166942335_f64) * t7153 + t2777 - t2780;
    t7441
}
