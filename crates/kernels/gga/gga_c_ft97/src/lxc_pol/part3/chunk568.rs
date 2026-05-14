//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 568/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk568<F: Float>(t291: F, t5266: F, t1208: F, t3780: F, t1701: F, t2725: F, t2730: F, t3796: F, t3804: F, t5031: F, t5034: F, t5039: F, t5043: F, t5047: F, t817: F, t1111: F, t1198: F, t1201: F, t1472: F, t2691: F, t285: F, t292: F, t4099: F, t5003: F, t5016: F, t5232: F, t5234: F, t5239: F, t5262: F, t5265: F) -> (F, F, F) {
    let t5267 = t5266 * t291;
    let t5272 = t3780 * t1208;
    let t5273 = t1701 * t5272;
    let t5284 = t1208 * t1208;
    let t5285 = t2725 * t5284;
    let t5295 = 0.48897200801234567903e0 * t5031 - 0.88904001456790123461e-1 * t3796 - 0.88904001456790123461e-1 * t5034 - t2730 + 0.11113000182098765433e-1 * t3804 + 0.22226000364197530865e-1 * t5039 - 0.33339000546296296298e-1 * t5043 + 0.16669500273148148149e-1 * t5047;
    let t5296 = t817 * t5295;
    let t5298 = 2.0 * t5232 - 0.2416365355361531912e1 * t5234 * t1111 + 0.2416365355361531912e1 * t1198 * t1111 - 4.0 * t2691 * t5239 + 2.0 * t5262 + 0.72985269132393279984e0 * t5265 * t5267 - 0.29194107652957311994e1 * t1201 * t5016 + 0.1208182677680765956e1 * t4099 * t5273 + 0.38259118126557588605e1 * t1201 * t5003 + 0.14597053826478655997e1 * t292 * t5016 - 0.1208182677680765956e1 * t1472 * t5273 - 0.38259118126557588605e1 * t292 * t5003 + 2.0 * t285 * t5285 - t285 * t5296;
    (t5284, t5295, t5298)
}
