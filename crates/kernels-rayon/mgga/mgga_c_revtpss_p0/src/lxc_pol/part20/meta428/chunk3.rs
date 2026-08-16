//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1610/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1610(t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64) -> f64 {
    let t44307 = 0.86419753086419753087e-1_f64 * t43813;
    let t44319 = t44307 - 0.19999999999999999999e0_f64 * t43854 + 0.44444444444444444444e-1_f64 * t43883 + 0.11111111111111111111e0_f64 * t43886 - 0.34567901234567901235e-1_f64 * t43888 + 0.22222222222222222222e-1_f64 * t43890 + 0.44444444444444444445e-1_f64 * t43892 - 0.66666666666666666668e-1_f64 * t43894 - 0.11111111111111111111e-1_f64 * t43896 - 0.19999999999999999999e0_f64 * t43899 + 0.2e0_f64 * t43902 + 0.83333333333333333333e-2_f64 * t43905;
    t44319
}
