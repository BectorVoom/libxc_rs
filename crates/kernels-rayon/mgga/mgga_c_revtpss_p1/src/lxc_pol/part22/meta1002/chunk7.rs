//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3416/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3416(t41281: f64, t41285: f64, t41287: f64, t41672: f64, t51937: f64, t51942: f64, t63266: f64, t63268: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64) -> f64 {
    let t64212 = -0.27785333333333333334e0_f64 * t51937 + 0.83356000000000000002e0_f64 * t51942 + t41672 + 0.10589175e2_f64 * t63266 - 0.6311625e0_f64 * t63268 + 0.23154444444444444444e0_f64 * t41281 - 0.11577222222222222222e0_f64 * t41285 - 0.3859074074074074074e-1_f64 * t41287 + 0.20659e1_f64 * t63274 - 0.68863333333333333333e0_f64 * t63276 + 0.22954444444444444444e0_f64 * t63278 - 0.68863333333333333334e0_f64 * t63281 - 0.34431666666666666667e0_f64 * t63285 - 0.57386111111111111112e0_f64 * t63290 + 0.20659e1_f64 * t63293;
    t64212
}
