//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 922/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk922(t2469: f64, t3842: f64, t729: f64, t2579: f64, t3977: f64, t1882: f64, t3856: f64, t3974: f64, t3972: f64, t242: f64, t10126: f64, t10128: f64, t10134: f64, t10140: f64, t10146: f64, t10148: f64, t14256: f64, t14261: f64, t14265: f64, t14269: f64, t446: f64) -> (f64, f64) {
    let t14273 = t729 * t2469 * t3842;
    let t14277 = t729 * t3977 * t2579;
    let t14281 = 2.0_f64 / 27.0_f64 * t1882 * t3856;
    let t14283 = 2.0_f64 / 9.0_f64 * t1882 * t3974;
    let t14288 = t2469 * t3972;
    let t14289 = t242 * t14288;
    let t14292 = t10126 / 27.0_f64 + 2.0_f64 / 81.0_f64 * t10128 + 2.0_f64 / 3.0_f64 * t446 * t14256 + t446 * t14261 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t14265 + 4.0_f64 / 3.0_f64 * t446 * t14269 + 2.0_f64 / 3.0_f64 * t446 * t14273 + 2.0_f64 / 3.0_f64 * t446 * t14277 + t14281 + t14283 - 8.0_f64 / 81.0_f64 * t10134 + 2.0_f64 / 27.0_f64 * t10140 - 2.0_f64 / 27.0_f64 * t10146 - 2.0_f64 / 9.0_f64 * t10148 - 2.0_f64 / 3.0_f64 * t446 * t14289;
    (t14288, t14292)
}
