//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 800/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk800(t13976: f64, t13981: f64, t18241: f64, t21402: f64, t21414: f64, t21419: f64, t21422: f64, t21433: f64, t21437: f64, t21448: f64, t21556: f64, t21567: f64, t21626: f64) -> f64 {
    let t21708 = -t18241 - t21402 / 3.0_f64 - 2.0_f64 * t21419 - t21556 / 4.0_f64 - t21414 / 9.0_f64 + 2.0_f64 * t21422 - 10.0_f64 / 81.0_f64 * t21433 - 2.0_f64 / 3.0_f64 * t21437 + 4.0_f64 / 9.0_f64 * t21448 - t13976 - t13981 + t21567 / 8.0_f64 + t21626 / 6.0_f64;
    t21708
}
