//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 792/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk792(t2262: f64, t2268: f64, t2271: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t9277: f64, t9289: f64, t9293: f64, t9296: f64, t9301: f64, t9305: f64, t9308: f64, t9311: f64) -> f64 {
    let t9312 = -1232.0_f64 / 27.0_f64 * t9277 * t44 + 220.0_f64 / 9.0_f64 * t2262 * t618 - 20.0_f64 / 9.0_f64 * t615 * t2268 - 20.0_f64 / 3.0_f64 * t615 * t2271 - 5.0_f64 / 108.0_f64 * t39 * t9289 + 5.0_f64 / 6.0_f64 * t39 * t9293 + 5.0_f64 / 6.0_f64 * t39 * t9296 + 5.0_f64 / 108.0_f64 * t51 * t9301 + 5.0_f64 / 6.0_f64 * t51 * t9305 - 5.0_f64 / 6.0_f64 * t51 * t9308 + t9311;
    t9312
}
