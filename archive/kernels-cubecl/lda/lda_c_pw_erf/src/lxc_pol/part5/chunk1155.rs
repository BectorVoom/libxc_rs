//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1155/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1155<F: Float>(t4753: F, t7720: F, t3416: F, t2411: F, t34: F, t1318: F, t4868: F, t1472: F, t7724: F, t2065: F, t3832: F, t571: F) -> (F, F, F, F, F, F) {
    let t21216 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4753 * t7720;
    let t21218 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3416 * t7720;
    let t21219 = t2411 * t34;
    let t21222 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1318 * t4868 * t21219;
    let t21224 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1472 * t7724;
    let t21228 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t571 * t3832 * t2411 * t2065;
    (t21216, t21218, t21219, t21222, t21224, t21228)
}
