//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1044/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1044(t57767: f64, t59426: f64, t59486: f64, t73439: f64, t73442: f64, t74307: f64, t74374: f64, t74377: f64, t86289: f64, t86297: f64, t86300: f64, t86303: f64, t86306: f64, t86309: f64) -> f64 {
    let t86477 = -t86289 / 6.0_f64 - t59426 + t57767 + 4.0_f64 / 3.0_f64 * t73439 + 2.0_f64 / 9.0_f64 * t73442 - 4.0_f64 / 9.0_f64 * t74307 + 4.0_f64 / 27.0_f64 * t74374 - 8.0_f64 / 9.0_f64 * t86297 - 4.0_f64 / 9.0_f64 * t86300 - 2.0_f64 / 3.0_f64 * t86303 + 2.0_f64 / 9.0_f64 * t86306 - 2.0_f64 / 3.0_f64 * t86309 + 20.0_f64 / 243.0_f64 * t74377 - t59486;
    t86477
}
