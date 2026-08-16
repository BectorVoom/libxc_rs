//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 838/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk838(t21973: f64, t2771: f64, t21588: f64, t848: f64, t21204: f64, t4206: f64, t10594: f64, t15011: f64, t15025: f64, t22302: f64, t22306: f64, t22310: f64, t22313: f64, t22316: f64, t22319: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64) {
    let t22321 = t2771 * t21973;
    let t22323 = t848 * t21588;
    let t22326 = t4206 * t21204;
    let t22329 = -2.0_f64 * t462 * t22302 - t10594 - t92 * t22306 - 4.0_f64 / 9.0_f64 * t15025 - 4.0_f64 / 3.0_f64 * t15011 + 2.0_f64 / 3.0_f64 * t462 * t22310 + 4.0_f64 / 3.0_f64 * t462 * t22313 - 2.0_f64 / 3.0_f64 * t462 * t22316 + t462 * t22319 + t462 * t22321 - 2.0_f64 * t462 * t22323 + 2.0_f64 * t462 * t22326;
    (t22321, t22323, t22326, t22329)
}
