//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1032/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1032<F: Float>(t11346: F, t6518: F, t758: F, t6526: F, t11153: F, t154: F, t907: F, t1220: F, t1224: F, t1238: F, t3214: F, t3846: F, t3849: F, t385: F, t3870: F, t3883: F, t3887: F, t3892: F, t6430: F, t6516: F, t6525: F, t8342: F, t8364: F, t8368: F) -> (F, F, F) {
    let t11422 = t11346 * t6518;
    let t11423 = t758 * t11422;
    let t11426 = t11346 * t6526;
    let t11427 = t758 * t11426;
    let t11439 = t154 * t907 * t11153;
    let t11444 = -F::cast_from(0.68598428988911579154e-2_f64) * t8368 * t3887 - F::cast_from(0.34299214494455789577e-2_f64) * t3214 * t3883 + F::cast_from(0.12862205435420921092e-2_f64) * t6516 * t11423 - F::cast_from(0.12862205435420921092e-2_f64) * t6525 * t11427 + t8342 / F::new(54.0) - F::cast_from(0.20579528696673473747e-1_f64) * t1238 * t3870 + t6430 - F::cast_from(0.14291339372689912324e-3_f64) * t8364 + t1220 * t3846 / F::new(12.0) - t1220 * t3892 / F::new(6.0) - t385 * t11439 / F::new(96.0) - F::new(11.0) / F::new(36.0) * t3849 * t1224;
    (t11422, t11426, t11444)
}
