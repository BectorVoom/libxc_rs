//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 743/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk743<F: Float>(t1954: F, t352: F, t4758: F, t1318: F, t2075: F, t518: F) -> (F, F, F, F) {
    let t4759 = t1954 * t352;
    let t4760 = t4758 * t4759;
    let t4762 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1318 * t4760;
    let t4763 = t2075 * t518;
    (t4759, t4760, t4762, t4763)
}
