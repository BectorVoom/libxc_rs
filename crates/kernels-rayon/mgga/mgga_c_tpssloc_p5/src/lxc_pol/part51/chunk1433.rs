//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1433/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1433(t102466: f64, t120340: f64, t120436: f64, t120533: f64, t122260: f64, t122270: f64, t122278: f64, t122281: f64, t16022: f64, t16460: f64, t26224: f64, t26482: f64, t31555: f64, t31653: f64, t5321: f64, t5326: f64, t6962: f64, t7194: f64, t8627: f64) -> f64 {
    let t122285 = -t120340 - 0.82246703342411321825e-2_f64 * t122260 + 2.0_f64 * t16022 * t8627 - t120436 - 6.0_f64 * t26224 * t102466 * t6962 - t120533 + 2.0_f64 * t31653 * t5326 + 0.16449340668482264365e-1_f64 * t122270 + 2.0_f64 * t5321 * t31555 + 2.0_f64 * t16460 * t8627 + 0.16449340668482264365e-1_f64 * t122278 - 0.82246703342411321825e-2_f64 * t122281 + 2.0_f64 * t7194 * t26482;
    t122285
}
