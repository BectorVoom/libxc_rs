//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 876/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk876(t41534: f64, t41536: f64, t235: f64, t9680: f64, t226: f64, t677: f64, t9682: f64, t122: f64, t196: f64, t9606: f64, t190: f64, t37991: f64) -> (f64, f64, f64, f64, f64) {
    let t41537 = t41534 * t41536;
    let t41547 = 1.0_f64 / t9680 / t235;
    let t41548 = t226 * t41547;
    let t41593 = t677 * t9682;
    let t41621 = t122 / t196 / t9606;
    let t41670 = 96.0_f64 * t37991 * t190;
    (t41537, t41548, t41593, t41621, t41670)
}
