//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1068/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1068<F: Float>(t1621: F, t1931: F, t4233: F, t838: F, t4714: F, t611: F, t348: F, t494: F, t4495: F, t3965: F, t4501: F, t4494: F) -> (F, F, F, F, F) {
    let t12507 = t1931 * t1621;
    let t12508 = F::cast_from(4.0_f64) * t12507;
    let t12509 = t838 * t4233;
    let t12514 = t4714 * t611;
    let t12516 = t348 * t494;
    let t12517 = t4495 * t12516;
    let t12520 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3965 * t4501 * t12517;
    let t12523 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3965 * t4494 * t12517;
    (t12508, t12509, t12514, t12520, t12523)
}
