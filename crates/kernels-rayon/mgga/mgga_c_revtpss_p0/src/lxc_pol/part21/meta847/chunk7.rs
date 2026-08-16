//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3182/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3182(t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64) -> f64 {
    let t58624 = 0.2283111111111111111e-1_f64 * t56228;
    let t58639 = 0.11415555555555555555e-1_f64 * t56212 + 0.6849333333333333333e-1_f64 * t56214 - 0.19025925925925925925e-1_f64 * t56216 + 0.57077777777777777775e-1_f64 * t56221 + 0.10274e0_f64 * t56226 + t58624 - 0.17123333333333333333e-1_f64 * t56230 + 0.17123333333333333333e-1_f64 * t56234 - 0.17757530864197530864e-1_f64 * t56236 - 0.63419753086419753085e-2_f64 * t43858 - 0.1522074074074074074e-1_f64 * t43865 + 0.2283111111111111111e-1_f64 * t43883 - 0.53272592592592592591e-1_f64 * t43888 + 0.2283111111111111111e-1_f64 * t43890 + 0.4566222222222222222e-1_f64 * t43892 - 0.34246666666666666665e-1_f64 * t43894 - 0.57077777777777777777e-2_f64 * t43896 + 0.57077777777777777775e-1_f64 * t56248 + 0.30822e0_f64 * t56252 - 0.20547999999999999999e0_f64 * t56256;
    t58639
}
