//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3667/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3667(t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68402: f64, t68464: f64) -> f64 {
    let t69279 = -0.68863333333333333332e0_f64 * t56187 - 0.20659e1_f64 * t56189 + 0.45908888888888888888e0_f64 * t56209 + 0.22954444444444444444e0_f64 * t56212 + 0.13772666666666666666e1_f64 * t56214 - 0.38257407407407407407e0_f64 * t56216 + 0.91817777777777777776e0_f64 * t56228 - 0.34431666666666666666e0_f64 * t56230 - 0.10712074074074074074e1_f64 * t56236 - 0.34431666666666666666e0_f64 * t68389 + 0.516475e0_f64 * t68393 - 0.68863333333333333334e0_f64 * t68397 + 0.45908888888888888889e0_f64 * t68399 + 0.46308888888888888889e-1_f64 * t68402 + 0.3529725e1_f64 * t68464;
    t69279
}
