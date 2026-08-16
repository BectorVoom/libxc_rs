//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1032/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1032(t11346: f64, t6518: f64, t758: f64, t6526: f64, t11153: f64, t154: f64, t907: f64, t1220: f64, t1224: f64, t1238: f64, t3214: f64, t3846: f64, t3849: f64, t385: f64, t3870: f64, t3883: f64, t3887: f64, t3892: f64, t6430: f64, t6516: f64, t6525: f64, t8342: f64, t8364: f64, t8368: f64) -> (f64, f64, f64) {
    let t11422 = t11346 * t6518;
    let t11423 = t758 * t11422;
    let t11426 = t11346 * t6526;
    let t11427 = t758 * t11426;
    let t11439 = t154 * t907 * t11153;
    let t11444 = -0.68598428988911579154e-2_f64 * t8368 * t3887 - 0.34299214494455789577e-2_f64 * t3214 * t3883 + 0.12862205435420921092e-2_f64 * t6516 * t11423 - 0.12862205435420921092e-2_f64 * t6525 * t11427 + t8342 / 54.0_f64 - 0.20579528696673473747e-1_f64 * t1238 * t3870 + t6430 - 0.14291339372689912324e-3_f64 * t8364 + t1220 * t3846 / 12.0_f64 - t1220 * t3892 / 6.0_f64 - t385 * t11439 / 96.0_f64 - 11.0_f64 / 36.0_f64 * t3849 * t1224;
    (t11422, t11426, t11444)
}
