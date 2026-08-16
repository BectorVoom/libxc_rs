//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1281/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1281(t49316: f64, t49318: f64, t49327: f64, t49329: f64, t49334: f64, t49344: f64, t49345: f64, t49347: f64, t49356: f64, t49362: f64, t49364: f64, t49371: f64, t49372: f64, t49378: f64, t49382: f64, t49387: f64, t49388: f64, t49399: f64, t49415: f64, t49471: f64, t49472: f64, t49478: f64) -> (f64, f64) {
    let t50565 = -t49316 - t49318 - t49327 - t49329 - t49334 + t49344 - t49345 - t49347 - t49356 - t49362 + t49364;
    let t50567 = t49371 + t49372 - t49378 + t49382 + t49387 + t49388 + t49399 + t49415 - t49471 - t49472 - t49478;
    (t50565, t50567)
}
