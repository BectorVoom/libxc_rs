//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 894/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk894<F: Float>(t6597: F, t942: F, t2422: F, t2430: F, t2454: F, t411: F, t415: F, t6536: F, t6548: F, t6552: F, t938: F, t952: F) -> (F, F) {
    let t6598 = t942 * t6597;
    let t6601 = F::cast_from(0.65854491829355115987e0_f64) * t6536 * t415 - F::cast_from(0.19756347548806534796e1_f64) * t2422 * t952 + F::cast_from(0.39512695097613069591e1_f64) * t938 * t2430 - F::cast_from(0.19756347548806534796e1_f64) * t938 * t2454 - F::cast_from(0.39512695097613069591e1_f64) * t411 * t6548 + F::cast_from(0.39512695097613069591e1_f64) * t411 * t6552 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t6598;
    (t6598, t6601)
}
