//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 891/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk891<F: Float>(t1499: F, t1555: F, t3155: F, t486: F, t1767: F, t206: F, t4068: F, t4077: F, t591: F, t4080: F, t4111: F, t4084: F) -> (F, F, F, F, F, F) {
    let t9402 = t1499 * t1555;
    let t9404 = t486 * t3155;
    let t9408 = F::cast_from(0.008082336938271605_f64) * t206 * t1767 * t4068;
    let t9410 = F::new(8.0) / F::new(9.0) * t4077 * t591;
    let t9412 = F::new(2e-21) * t4080 * t4111;
    let t9413 = t4084 * t591;
    (t9402, t9404, t9408, t9410, t9412, t9413)
}
