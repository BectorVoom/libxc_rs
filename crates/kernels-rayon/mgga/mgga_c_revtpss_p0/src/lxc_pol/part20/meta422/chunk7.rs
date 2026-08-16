//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1588/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1588(t43854: f64, t43881: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64) -> f64 {
    let t43907 = t43881 - 8.0_f64 * t43854 + 16.0_f64 / 9.0_f64 * t43883 + 40.0_f64 / 9.0_f64 * t43886 - 112.0_f64 / 81.0_f64 * t43888 + 8.0_f64 / 9.0_f64 * t43890 + 16.0_f64 / 9.0_f64 * t43892 - 8.0_f64 / 3.0_f64 * t43894 - 4.0_f64 / 9.0_f64 * t43896 - 8.0_f64 * t43899 + 8.0_f64 * t43902 + t43905 / 3.0_f64;
    t43907
}
