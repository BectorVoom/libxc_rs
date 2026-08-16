//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1013/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1013(t1948: f64, t2252: f64, t342: f64, t142: f64, t7763: f64, t511: f64, t8639: f64, t11269: f64, t1526: f64, t1527: f64, t15567: f64, t16640: f64, t1943: f64, t343: f64, t72: f64, t7745: f64, t7765: f64, t7789: f64, t8766: f64, t9007: f64, t9041: f64, t9045: f64, t9078: f64, t9084: f64) -> f64 {
    let t41305 = t342 * t2252 * t1948;
    let t41318 = t142 * t7763;
    let t41328 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t511;
    let t41329 = t1526 * t1527 * t9078 / 2.0_f64 - t1526 * t1527 * t8766 * t7765 / 2.0_f64 + t15567 * t16640 * t7789 / 2.0_f64 + t41305 / 6.0_f64 + t9084 - t342 * t343 * t72 * t9007 / 4.0_f64 - t1526 * t1527 * t1943 * t7745 / 12.0_f64 - t1526 * t1527 * t9041 / 4.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t11269 * t41318 * t7765 - t1526 * t1527 * t9045 / 4.0_f64 - t41328;
    t41329
}
