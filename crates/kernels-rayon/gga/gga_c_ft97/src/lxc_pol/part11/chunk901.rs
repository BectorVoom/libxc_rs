//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 901/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk901(t2: f64, t38463: f64, t355: f64, t7241: f64, t369: f64, t7760: f64, t1788: f64, t8282: f64, t458: f64, t8305: f64, t11690: f64, t1787: f64, t3127: f64, t3134: f64, t37273: f64, t37283: f64, t37306: f64, t37311: f64, t37315: f64, t37320: f64, t38264: f64, t38269: f64, t38273: f64, t38283: f64, t38461: f64, t462: f64, t8291: f64, t8327: f64) -> (f64, f64, f64) {
    let t38464 = t38463 * t2;
    let t38477 = t355 * t7241;
    let t38478 = t38477 * t2;
    let t38482 = t7760 * t369;
    let t38483 = t38482 * t2;
    let t38490 = t8282 * t1788;
    let t38501 = t458 * t8305;
    let t38503 = -4.0_f64 / 3.0_f64 * t38461 - 8.0_f64 / 3.0_f64 * t462 * t38464 * t38269 - 4.0_f64 * t462 * t1787 * t38283 + 8.0_f64 * t462 * t8291 * t37273 - 16.0_f64 / 3.0_f64 * t462 * t8327 * t38273 + 8.0_f64 * t462 * t38478 * t38264 + 40.0_f64 / 27.0_f64 * t462 * t38483 * t37306 - 20.0_f64 / 9.0_f64 * t462 * t11690 * t37311 + 16.0_f64 / 9.0_f64 * t38490 + 2.0_f64 * t462 * t1787 * t37283 - 12.0_f64 * t462 * t3134 * t37315 + 8.0_f64 * t462 * t3127 * t37320 + 4.0_f64 / 3.0_f64 * t38501;
    (t38477, t38482, t38503)
}
