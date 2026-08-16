//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 603/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk603(t27789: f64, t6118: f64, t24543: f64, t6884: f64, t24483: f64, t24485: f64, t24500: f64, t24517: f64, t24524: f64, t27765: f64, t27769: f64, t27773: f64, t27778: f64, t27783: f64) -> (f64, f64, f64) {
    let t27790 = t6118 * t27789;
    let t27792 = t24543 * t6884;
    let t27794 = t27765 / 9.0_f64 - t27769 / 3.0_f64 - t27773 / 12.0_f64 - t27778 / 12.0_f64 - t24483 + t24485 / 3.0_f64 - t27783 - 2.0_f64 / 3.0_f64 * t24500 + t24517 / 6.0_f64 - t24524 / 9.0_f64 + t27790 / 6.0_f64 - t27792 / 18.0_f64;
    (t27790, t27792, t27794)
}
