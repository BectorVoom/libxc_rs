//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1105/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1105(t11571: f64, t324: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> (f64, f64) {
    let t11572 = t11571 * t324;
    let t11574 = 0.53272592592592592592e-1_f64 * t11132;
    let t11585 = -t11574 - 0.2283111111111111111e-1_f64 * t11134 + 0.11415555555555555555e-1_f64 * t11136 - 0.34246666666666666665e-1_f64 * t11138 + 0.17123333333333333333e-1_f64 * t11140 - 0.19025925925925925925e-1_f64 * t11147 + 0.68493333333333333331e-1_f64 * t11153 - 0.34246666666666666665e-1_f64 * t11158 - 0.10274e0_f64 * t11162 + 0.10274e0_f64 * t11167 - 0.17123333333333333333e-1_f64 * t11171;
    (t11572, t11585)
}
