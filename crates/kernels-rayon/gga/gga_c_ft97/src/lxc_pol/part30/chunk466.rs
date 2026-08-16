//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 466/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk466(t676: f64, t7484: f64, t27: f64, t89: f64, t7518: f64, t7522: f64, t7526: f64, t7530: f64) -> (f64, f64, f64) {
    let t7532 = t676 * t7484;
    let t7534 = t89 * t27 * t7532;
    let t7536 = -t7518 / 3.0_f64 + t7522 / 3.0_f64 - t7526 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t7530 - t7534 / 3.0_f64;
    (t7532, t7534, t7536)
}
