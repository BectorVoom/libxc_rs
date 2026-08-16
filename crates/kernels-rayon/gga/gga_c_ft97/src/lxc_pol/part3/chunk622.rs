//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 622/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk622(t5295: f64, t817: f64, t1111: f64, t1198: f64, t1201: f64, t1472: f64, t2691: f64, t285: f64, t292: f64, t4099: f64, t5003: f64, t5016: f64, t5232: f64, t5234: f64, t5239: f64, t5262: f64, t5265: f64, t5267: f64, t5273: f64, t5285: f64) -> f64 {
    let t5296 = t817 * t5295;
    let t5298 = 2.0_f64 * t5232 - 0.2416365355361531912e1_f64 * t5234 * t1111 + 0.2416365355361531912e1_f64 * t1198 * t1111 - 4.0_f64 * t2691 * t5239 + 2.0_f64 * t5262 + 0.72985269132393279984e0_f64 * t5265 * t5267 - 0.29194107652957311994e1_f64 * t1201 * t5016 + 0.1208182677680765956e1_f64 * t4099 * t5273 + 0.38259118126557588605e1_f64 * t1201 * t5003 + 0.14597053826478655997e1_f64 * t292 * t5016 - 0.1208182677680765956e1_f64 * t1472 * t5273 - 0.38259118126557588605e1_f64 * t292 * t5003 + 2.0_f64 * t285 * t5285 - t285 * t5296;
    t5298
}
