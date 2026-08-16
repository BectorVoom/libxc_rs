//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 843/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk843(t16323: f64, t6: f64, t6879: f64, t161: f64, t2024: f64, t16220: f64, t16249: f64, t16251: f64, t16252: f64, t6318: f64, t6321: f64, t6324: f64, t6328: f64, t6330: f64, t6356: f64, t6526: f64, t6619: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16324 = t6 * t16323;
    let t16325 = t16324 * t6879;
    let t16326 = t161 * t16325;
    let t16329 = t16324 * t2024;
    let t16330 = t161 * t16329;
    let t16333 = -t6318 - t6321 - t6324 - t6328 - t6330 - t16220 + t6526 + t16249 - t6356 - t16251 - t6619 - t16252;
    (t16324, t16325, t16326, t16329, t16330, t16333)
}
