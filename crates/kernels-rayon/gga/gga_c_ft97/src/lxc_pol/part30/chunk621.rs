//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 621/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk621(t28037: f64, t3886: f64, t28036: f64, t6752: f64, t684: f64, t24231: f64, t24455: f64, t24470: f64, t27466: f64, t27471: f64, t27473: f64, t27477: f64, t27481: f64, t27485: f64, t27745: f64, t27751: f64, t27755: f64, t27759: f64) -> (f64, f64, f64, f64, f64) {
    let t28038 = t28037 * t3886;
    let t28039 = t28036 * t28038;
    let t28042 = t6752 * t684;
    let t28043 = t24231 * t28042;
    let t28057 = t27466 / 18.0_f64 + t27471 / 9.0_f64 - t27473 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t27477 - 2.0_f64 * t27481 + t27485 / 9.0_f64 - t27745 / 6.0_f64 - t24455 / 36.0_f64 - t24470 / 9.0_f64 - t27751 - t27755 / 9.0_f64 - t27759 / 9.0_f64;
    (t28038, t28039, t28042, t28043, t28057)
}
