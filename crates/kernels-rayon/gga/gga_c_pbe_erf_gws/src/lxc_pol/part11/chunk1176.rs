//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1176/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1176(t26204: f64, t47355: f64, t47359: f64, t47363: f64, t47364: f64, t47366: f64, t47368: f64, t47369: f64, t47376: f64, t47381: f64, t47385: f64, t16553: f64, t16556: f64, t16561: f64, t47387: f64, t47389: f64, t47506: f64, t47507: f64, t47511: f64, t47515: f64, t47519: f64, t47523: f64) -> (f64, f64) {
    let t48625 = -t47355 - t47359 - t47363 - t47364 + t47366 + 0.86568330898918747016e0_f64 * t26204 - t47368 - t47369 + t47376 + t47381 - t47385;
    let t48626 = -t47387 - t47389 - t47506 - t47507 - t47511 - t47515 + t47519 - t47523 + t16553 + t16556 - t16561;
    (t48625, t48626)
}
