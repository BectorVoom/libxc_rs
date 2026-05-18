//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1213/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1213<F: Float>(t11810: F, t11813: F, t19311: F, t19313: F, t19419: F, t19421: F, t19423: F, t19424: F, t19425: F, t19426: F, t19428: F, t19431: F) -> F {
    let t21901 = F::new(0.004546314527777778) * t11810 - F::new(0.040518518518518516) * t11813 - t19311 - t19313 + t19419 + t19421 + t19423 - t19424 - t19425 + t19426 - t19428 - t19431;
    t21901
}
