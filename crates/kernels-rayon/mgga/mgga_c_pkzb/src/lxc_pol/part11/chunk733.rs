//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 733/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk733(t197: f64, t5718: f64, t2021: f64, t271: f64, t296: f64, t294: f64, t46: f64, t133: f64, t2029: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5719 = t5718 * t197;
    let t5722 = 1.0_f64 / t2021 / t296 / t271;
    let t5723 = t294 * t5722;
    let t5724 = t5723 * t46;
    let t5725 = t5719 * t5724;
    let t5728 = t2029 * t133;
    (t5719, t5722, t5723, t5724, t5725, t5728)
}
