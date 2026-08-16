//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1636/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1636(t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64) -> f64 {
    let t44865 = 0.15365432098765432099e0_f64 * t43813;
    let t44877 = t44865 - 0.35560000000000000001e0_f64 * t43854 + 0.79022222222222222224e-1_f64 * t43883 + 0.19755555555555555556e0_f64 * t43886 - 0.61461728395061728396e-1_f64 * t43888 + 0.39511111111111111112e-1_f64 * t43890 + 0.79022222222222222224e-1_f64 * t43892 - 0.11853333333333333334e0_f64 * t43894 - 0.19755555555555555556e-1_f64 * t43896 - 0.35560000000000000001e0_f64 * t43899 + 0.35560000000000000001e0_f64 * t43902 + 0.14816666666666666667e-1_f64 * t43905;
    t44877
}
