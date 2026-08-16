//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 858/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk858(t22: f64, t37991: f64, t25: f64, t30: f64, t17: f64, t37352: f64, t11401: f64, t23: f64, t26: f64, t11: f64, t1690: f64, t32: f64, t8991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37993 = 96.0_f64 * t37991 * t22;
    let t37996 = t25 / t30 / t37993;
    let t38052 = t17 * t37352;
    let t38061 = t11401 * t23;
    let t38062 = t26 * t38061;
    let t38063 = 280.0_f64 / 81.0_f64 * t38062;
    let t38176 = t1690 * t11;
    let t38200 = t8991 / t32;
    (t37993, t37996, t38052, t38061, t38062, t38063, t38176, t38200)
}
