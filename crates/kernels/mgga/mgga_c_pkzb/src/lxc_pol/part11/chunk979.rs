//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 979/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk979<F: Float>(t10727: F, t626: F, t1045: F, t1055: F, t10676: F, t10686: F, t10689: F, t184: F, t188: F, t3461: F, t3467: F, t3488: F) -> (F, F) {
    let t10728 = t626 * t10727;
    let t10731 = F::cast_from(0.65854491829355115987e0_f64) * t10676 * t188 - F::cast_from(0.19756347548806534796e1_f64) * t3461 * t1055 + F::cast_from(0.39512695097613069591e1_f64) * t1045 * t3467 - F::cast_from(0.19756347548806534796e1_f64) * t1045 * t3488 - F::cast_from(0.39512695097613069591e1_f64) * t184 * t10686 + F::cast_from(0.39512695097613069591e1_f64) * t184 * t10689 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t10728;
    (t10728, t10731)
}
