//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1210/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1210<F: Float>(t11153: F, t11156: F, t11159: F, t11166: F, t11168: F, t13420: F, t21771: F, t21775: F, t21776: F, t21871: F, t21875: F, t21878: F, t21881: F) -> F {
    let t21882 = t21771 + t21775 - t21776 + t21871 - t13420 - t11153 - t11156 + t11159 - F::new(2.0) / F::new(9.0) * t11166 - F::cast_from(0.013506172839506173_f64) * t11168 - t21875 + t21878 - t21881;
    t21882
}
