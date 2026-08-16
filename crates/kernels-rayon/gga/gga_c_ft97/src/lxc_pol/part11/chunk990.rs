//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 990/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk990(t165: f64, t39641: f64, t39646: f64, t39649: f64, t39655: f64, t39658: f64, t40517: f64, t40519: f64, t40522: f64, t40525: f64, t40540: f64, t40555: f64, t40570: f64, t40585: f64, t515: f64, t564: f64, t9460: f64) -> f64 {
    let t40590 = -12.0_f64 * t39641 - 4.0_f64 * t564 * t9460 + 16.0_f64 * t39646 + 12.0_f64 * t39649 + 48.0_f64 * t39655 - 72.0_f64 * t39658 - 2.0_f64 * t40517 - 8.0_f64 * t40519 - 8.0_f64 * t40522 + 24.0_f64 * t40525 - t515 * (t40540 + t40555 + t40570 + t40585) * t165;
    t40590
}
