//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 808/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk808(t103: f64, t16533: f64, t16085: f64, t16089: f64, t16199: f64, t16204: f64, t16215: f64, t16247: f64, t16251: f64, t16279: f64, t16292: f64, t16481: f64) -> f64 {
    let t16550 = t16533 * t103;
    let t16562 = 2.0_f64 * t16550 - 2.0_f64 * t16247 - 4.0_f64 * t16085 + 8.0_f64 * t16279 - 4.0_f64 * t16089 + 4.0_f64 * t16251 - 12.0_f64 * t16199 + 8.0_f64 * t16204 - 2.0_f64 * t16215 + 4.0_f64 * t16292 - 2.0_f64 * t16481;
    t16562
}
