//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 433/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk433<F: Float>(t5295: F, t817: F, t1111: F, t1198: F, t1201: F, t1472: F, t2691: F, t285: F, t292: F, t4099: F, t5003: F, t5016: F, t5232: F, t5234: F, t5239: F, t5262: F, t5265: F, t5267: F, t5273: F, t5285: F) -> (F,) {
    let t5296 = t817 * t5295;
    let t5298 = 2.0 * t5232 - 0.2416365355361531912e1 * t5234 * t1111 + 0.2416365355361531912e1 * t1198 * t1111 - 4.0 * t2691 * t5239 + 2.0 * t5262 + 0.72985269132393279984e0 * t5265 * t5267 - 0.29194107652957311994e1 * t1201 * t5016 + 0.1208182677680765956e1 * t4099 * t5273 + 0.38259118126557588605e1 * t1201 * t5003 + 0.14597053826478655997e1 * t292 * t5016 - 0.1208182677680765956e1 * t1472 * t5273 - 0.38259118126557588605e1 * t292 * t5003 + 2.0 * t285 * t5285 - t285 * t5296;
    (t5298,)
}
