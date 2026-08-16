//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2522/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2522(t2718: f64, t4469: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64, t2783: f64, t786: f64, t10073: f64, t14588: f64, t10542: f64, t14563: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51396 = t2718 * t4469;
    let t51403 = t9292 * t4519;
    let t51408 = t2798 * t4499 * t9288;
    let t51421 = t786 * t2783 * t4469;
    let t51424 = t10073 * t14588;
    let t51429 = t10542 * t14563;
    (t51396, t51403, t51408, t51421, t51424, t51429)
}
