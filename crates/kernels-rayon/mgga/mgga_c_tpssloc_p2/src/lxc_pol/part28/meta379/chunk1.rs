//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1448/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1448(t14704: f64, t14710: f64, t14722: f64, t11215: f64, t11217: f64, t14720: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64) -> (f64, f64, f64) {
    let t14868 = 0.19931111111111111111e0_f64 * t14704;
    let t14870 = 0.10954222222222222222e0_f64 * t14710;
    let t14886 = 0.39862222222222222222e0_f64 * t14722;
    let t14887 = -0.10954222222222222222e0_f64 * t11215 - 0.54771111111111111111e-1_f64 * t11217 + 0.91285185185185185185e-1_f64 * t14766 + 0.13287407407407407408e0_f64 * t14720 - 0.39862222222222222222e0_f64 * t14738 - 0.19931111111111111111e0_f64 * t14742 - 0.11958666666666666667e1_f64 * t14733 + 0.11958666666666666667e1_f64 * t14751 + 0.59793333333333333334e0_f64 * t14755 + 0.17938e1_f64 * t14746 - t14886;
    (t14868, t14870, t14887)
}
