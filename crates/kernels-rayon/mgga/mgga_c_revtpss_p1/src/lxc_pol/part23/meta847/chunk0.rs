//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2728/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2728(t12916: f64, t20837: f64, t5331: f64, t12910: f64, t21003: f64, t12809: f64, t21029: f64, t21177: f64, t3678: f64, t17303: f64, t5327: f64, t11249: f64, t1248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70685 = t5331 * t12916 * t20837;
    let t70689 = t12910 * t12916 * t21003;
    let t70733 = t12809 * t12916 * t21029;
    let t70756 = t21177 * t3678;
    let t70758 = t5327 * t17303;
    let t70794 = t11249 * t1248;
    (t70685, t70689, t70733, t70756, t70758, t70794)
}
