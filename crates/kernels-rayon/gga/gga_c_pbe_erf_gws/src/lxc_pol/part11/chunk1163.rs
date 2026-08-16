//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1163/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1163(t40: f64, t48472: f64, t87: f64, t42448: f64, t18477: f64, t18479: f64, t18512: f64, t18518: f64, t48440: f64, t48441: f64, t48442: f64, t48443: f64, t48444: f64, t48445: f64, t48446: f64) -> (f64, f64, f64) {
    let t48474 = t40 * t48472 * t87;
    let t48475 = 16.0_f64 * t42448;
    let t48476 = t18477 + t48440 - t18479 + t48441 + t18512 + t18518 + t48442 - t48443 + t48444 + t48445 - t48446 + t48474 - t48475;
    (t48474, t48475, t48476)
}
