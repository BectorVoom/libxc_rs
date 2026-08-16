//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 944/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk944(t1580: f64, t184: f64, t185: f64, t21: f64, t2236: f64, t2240: f64, t2301: f64, t2306: f64, t363: f64, t37391: f64, t39390: f64, t39396: f64, t39438: f64, t39481: f64, t39574: f64, t39624: f64, t5: f64, t620: f64, t623: f64, t649: f64, t650: f64, t7745: f64, t8614: f64, t8723: f64, t8731: f64, t8732: f64, t9471: f64) -> f64 {
    let t39637 = 3.0_f64 / 2.0_f64 * t5 * t2236 * t1580 + t5 * t185 * t37391 / 4.0_f64 + t623 * t649 * t7745 + t5 * t620 * t7745 + 3.0_f64 / 2.0_f64 * t8614 * t2306 + t623 * t8731 * t363 + t623 * t39390 * t184 * t21 / 4.0_f64 + t2240 * t8732 + t39396 * t650 + t623 * (t39438 + t39481 + t39574 + t39624) * t184 * t21 / 4.0_f64 + t623 * t8723 * t363 + t5 * t9471 * t363 + 3.0_f64 / 2.0_f64 * t8614 * t2301;
    t39637
}
