//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 835/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk835(t1096: f64, t33356: f64, t33357: f64, t1418: f64, t35415: f64, t39: f64, t6018: f64, t3766: f64, t22511: f64, t6817: f64, t213: f64, t230: f64) -> (f64, f64, f64, f64, f64) {
    let t35446 = t33356 * t33357 * t1096;
    let t35449 = t1418 * t35415;
    let t35452 = t6018 * t39;
    let t35453 = t3766 * t35452;
    let t35454 = t22511 * t6817;
    let t35455 = t230 * t213;
    (t35446, t35449, t35453, t35454, t35455)
}
