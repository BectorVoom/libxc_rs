//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 752/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk752(t21027: f64, t21031: f64, t21036: f64, t21040: f64, t21044: f64, t21048: f64, t21052: f64, t21056: f64, t21059: f64, t21062: f64, t21064: f64, t2265: f64, t631: f64) -> f64 {
    let t21066 = t631 * t21027 / 2.0_f64 + t631 * t21031 / 6.0_f64 + 6.0_f64 * t631 * t21036 - 9.0_f64 / 2.0_f64 * t631 * t21040 + 2.0_f64 / 27.0_f64 * t631 * t21044 + 3.0_f64 * t2265 * t21048 + 2.0_f64 * t2265 * t21052 - t2265 * t21056 - t2265 * t21059 + t631 * t21062 - t2265 * t21064;
    t21066
}
