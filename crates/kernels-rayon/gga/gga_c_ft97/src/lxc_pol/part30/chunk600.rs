//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 600/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk600(t24438: f64, t27757: f64, t6118: f64, t24455: f64, t24470: f64, t27466: f64, t27471: f64, t27473: f64, t27477: f64, t27481: f64, t27485: f64, t27745: f64, t27751: f64, t27755: f64) -> (f64, f64) {
    let t27758 = t24438 * t27757;
    let t27759 = t6118 * t27758;
    let t27761 = t27466 / 6.0_f64 + t27471 / 3.0_f64 - t27473 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t27477 - 6.0_f64 * t27481 + t27485 / 3.0_f64 - t27745 / 2.0_f64 - t24455 / 12.0_f64 - t24470 / 3.0_f64 - 3.0_f64 * t27751 - t27755 / 3.0_f64 - t27759 / 3.0_f64;
    (t27759, t27761)
}
