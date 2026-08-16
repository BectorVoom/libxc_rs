//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 813/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk813(t2749: f64, t7629: f64, t840: f64, t681: f64, t7664: f64, t89: f64, t296: f64, t34013: f64, t7686: f64, t824: f64, t1901: f64, t193: f64, t34199: f64, t34204: f64, t34209: f64, t34213: f64, t34217: f64, t34221: f64, t34227: f64, t34232: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t34236 = t840 * t2749 * t7629;
    let t34241 = t89 * t681 * t7664 / 9.0_f64;
    let t34242 = t296 * t34013;
    let t34246 = t840 * t7686 * t824;
    let t34249 = -2.0_f64 / 9.0_f64 * t1901 * t34199 + t1901 * t34204 / 9.0_f64 + t1901 * t34209 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t34213 - 2.0_f64 / 3.0_f64 * t446 * t34217 + t89 * t193 * t34221 / 3.0_f64 + t446 * t34227 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t34232 + 2.0_f64 / 3.0_f64 * t446 * t34236 - t34241 - t446 * t34242 / 3.0_f64 - t446 * t34246 / 3.0_f64;
    (t34236, t34241, t34242, t34246, t34249)
}
