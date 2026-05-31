//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1211/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1211<F: Float>(t18188: F, t2006: F, t3965: F, t16657: F, t1996: F, t4488: F, t17883: F, t17886: F, t1308: F, t571: F, t593: F, t7422: F) -> (F, F, F, F, F) {
    let t21885 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t18188 * t2006;
    let t21888 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4488 * t16657 * t1996;
    let t21889 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t17883;
    let t21890 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17886;
    let t21894 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t571 * t1308 * t7422 * t593;
    (t21885, t21888, t21889, t21890, t21894)
}
