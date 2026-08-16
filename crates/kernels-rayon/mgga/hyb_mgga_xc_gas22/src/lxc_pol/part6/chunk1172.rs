//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1172/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1172(t7074: f64, t944: f64, t19746: f64, t222: f64, t343: f64, t6007: f64, t940: f64) -> (f64, f64, f64, f64, f64) {
    let t21382 = t944 * t7074;
    let t21389 = t222 * t19746 * t343;
    let t21390 = 0.31310740740740740741e1_f64 * t21389;
    let t21391 = 280.0_f64 / 81.0_f64 * t21389;
    let t21393 = t222 * t6007 * t940;
    (t21382, t21389, t21390, t21391, t21393)
}
