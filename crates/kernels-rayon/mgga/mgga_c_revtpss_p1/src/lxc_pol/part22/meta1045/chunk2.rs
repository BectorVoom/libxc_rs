//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3661/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3661(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64) -> f64 {
    let t69181 = -0.11415555555555555555e-1_f64 * t56230 - 0.35515061728395061727e-1_f64 * t56236 - 0.11415555555555555555e-1_f64 * t68389 + 0.17123333333333333333e-1_f64 * t68393 - 0.2283111111111111111e-1_f64 * t68397 + 0.1522074074074074074e-1_f64 * t68399 - 0.50735802469135802469e-2_f64 * t43865 - 0.35515061728395061728e-1_f64 * t43888 + 0.76103703703703703703e-2_f64 * t43890 + 0.15220740740740740741e-1_f64 * t43892 - 0.45662222222222222221e-1_f64 * t68454 - 0.68493333333333333332e-1_f64 * t68456 + 0.10274e0_f64 * t68459;
    t69181
}
