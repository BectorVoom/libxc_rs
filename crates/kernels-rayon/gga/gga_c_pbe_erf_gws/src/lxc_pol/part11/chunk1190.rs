//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1190/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1190(t225: f64, t231: f64, t48321: f64, t48369: f64, t48373: f64, t48377: f64, t48380: f64, t48381: f64, t48382: f64, t48387: f64, t48392: f64, t48393: f64, t48394: f64) -> f64 {
    let t48694 = t48369 + t48373 + t48377 + t48380 - t48381 + t48382 + t48387 - t48392 + t48393 + 4.0_f64 / 3.0_f64 * t48321 * t225 * t231 - t48394;
    t48694
}
