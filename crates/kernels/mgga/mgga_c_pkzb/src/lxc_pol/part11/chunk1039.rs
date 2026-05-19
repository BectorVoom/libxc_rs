//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1039/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1039<F: Float>(t11532: F, t942: F, t11484: F, t11494: F, t11497: F, t1246: F, t1256: F, t3904: F, t3910: F, t3929: F, t411: F, t415: F) -> (F, F) {
    let t11533 = t942 * t11532;
    let t11536 = F::cast_from(0.65854491829355115987e0_f64) * t11484 * t415 - F::cast_from(0.19756347548806534796e1_f64) * t3904 * t1256 + F::cast_from(0.39512695097613069591e1_f64) * t1246 * t3910 - F::cast_from(0.19756347548806534796e1_f64) * t1246 * t3929 - F::cast_from(0.39512695097613069591e1_f64) * t411 * t11494 + F::cast_from(0.39512695097613069591e1_f64) * t411 * t11497 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t11533;
    (t11533, t11536)
}
