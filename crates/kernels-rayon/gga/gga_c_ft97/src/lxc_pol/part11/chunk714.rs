//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 714/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk714(t89: f64, t9716: f64, t9718: f64, t2336: f64, t2366: f64, t2344: f64, t375: f64, t2350: f64, t2374: f64, t9520: f64, t9695: f64, t9699: f64, t9701: f64, t9705: f64, t9711: f64, t9715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9720 = t89 * t9716 * t9718;
    let t9723 = t89 * t2336 * t2366;
    let t9725 = t375 * t2344;
    let t9727 = t89 * t9725 * t2350;
    let t9730 = t89 * t375 * t2374;
    let t9732 = t9520 / 6.0_f64 - t9695 / 6.0_f64 - t9699 - 2.0_f64 / 9.0_f64 * t9701 - t9705 / 18.0_f64 - t9711 + t9715 - 5.0_f64 / 81.0_f64 * t9720 + t9723 / 18.0_f64 + t9727 / 27.0_f64 - t9730 / 3.0_f64;
    (t9720, t9723, t9725, t9727, t9730, t9732)
}
