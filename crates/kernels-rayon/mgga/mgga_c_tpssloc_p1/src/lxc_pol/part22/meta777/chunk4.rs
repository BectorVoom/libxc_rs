//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2659/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659(t25: f64, t1298: f64, t15989: f64, t15992: f64, t16557: f64, t19606: f64, t20216: f64, t20376: f64, t2219: f64, t3704: f64, t39861: f64, t5170: f64, t606: f64, t67059: f64, t73975: f64, t73978: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t74335 = piecewise3(t26, 0.0_f64, -56.0_f64 / 81.0_f64 * t39861 * t20376 * t606 + 16.0_f64 / 9.0_f64 * t19606 * t2219 + 8.0_f64 / 9.0_f64 * t15989 * t73975 - 4.0_f64 / 3.0_f64 * t15992 * t73978 - 2.0_f64 / 3.0_f64 * t5170 * t16557 - 2.0_f64 / 9.0_f64 * t3704 * t20216 * t606 + 2.0_f64 / 3.0_f64 * t1298 * t67059);
    t74335
}
