//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 746/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk746(t2610: f64, t8392: f64, t2526: f64, t761: f64, t684: f64, t2606: f64, t9520: f64, t9695: f64, t9701: f64, t9705: f64, t9711: f64, t9715: f64, t9720: f64, t9723: f64, t9727: f64, t9730: f64, t9735: f64, t9739: f64, t9752: f64) -> (f64, f64, f64, f64, f64) {
    let t10090 = t8392 * t2610;
    let t10092 = t761 * t2526;
    let t10093 = t10092 * t684;
    let t10094 = t2606 * t10093;
    let t10108 = -t9705 / 3.0_f64 + 6.0_f64 * t9715 - 10.0_f64 / 27.0_f64 * t9720 + t9723 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t9727 - 4.0_f64 / 9.0_f64 * t9735 - 2.0_f64 * t9739 + 4.0_f64 / 3.0_f64 * t9752 - t9695 - 4.0_f64 / 3.0_f64 * t9701 - 6.0_f64 * t9711 - 2.0_f64 * t9730 + t9520;
    (t10090, t10092, t10093, t10094, t10108)
}
