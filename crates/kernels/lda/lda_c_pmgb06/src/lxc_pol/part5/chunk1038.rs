//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1038/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1038<F: Float>(t1423: F, t7555: F, t19307: F, t19309: F, t19311: F, t19313: F, t19419: F, t19421: F, t19423: F, t19424: F, t19425: F, t19426: F) -> (F, F) {
    let t19427 = t1423 * t7555;
    let t19428 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t19427;
    let t19429 = -t19307 + t19309 - t19311 - t19313 + t19419 + t19421 + t19423 - t19424 - t19425 + t19426 - t19428;
    (t19428, t19429)
}
