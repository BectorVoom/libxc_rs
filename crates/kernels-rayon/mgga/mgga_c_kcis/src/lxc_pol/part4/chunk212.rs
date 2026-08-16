//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 212/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk212(t81: f64, t60: f64, t64: f64, t8: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t686 = t81 * t81;
    let t687 = 1.0_f64 / t686;
    let t688 = t60 * t687;
    let t689 = t64 * t8;
    let t691 = 1.0_f64 / t9 / t689;
    (t686, t687, t688, t689, t691)
}
