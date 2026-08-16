//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 611/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk611(t1557: f64, t8654: f64, t1736: f64, t179: f64, t1068: f64, t8640: f64, t171: f64, t7741: f64, t1075: f64, t7773: f64, t89: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12122 = t8654 * t1557;
    let t12137 = t1736 * t179;
    let t12165 = t8640 * t1068;
    let t12168 = 1.0_f64 / t171 / t7741;
    let t12204 = t8640 * t1075;
    let t12362 = t89 * t7773 * t998;
    (t12122, t12137, t12165, t12168, t12204, t12362)
}
