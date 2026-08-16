//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 665/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk665(t24: f64, t9017: f64, t9236: f64, t2118: f64, t458: f64, t462: f64, t92: f64, t9205: f64, t9207: f64, t9209: f64, t9211: f64, t9214: f64, t9218: f64, t9221: f64, t9225: f64, t9230: f64, t9233: f64) -> (f64, f64) {
    let t9238 = t24 * t9236 * t9017;
    let t9241 = t458 * t2118;
    let t9242 = t9205 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t9207 - 2.0_f64 * t9209 - 2.0_f64 * t462 * t9211 + 2.0_f64 * t462 * t9214 - 2.0_f64 * t462 * t9218 - 2.0_f64 * t462 * t9221 - 10.0_f64 / 27.0_f64 * t462 * t9225 + 6.0_f64 * t462 * t9230 - t462 * t9233 / 3.0_f64 - 6.0_f64 * t92 * t9238 + t9241;
    (t9238, t9242)
}
