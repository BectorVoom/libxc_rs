//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1175/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1175(t16485: f64, t16490: f64, t16498: f64, t16501: f64, t26196: f64, t47293: f64, t47297: f64, t47299: f64, t47301: f64, t47303: f64, t47307: f64, t47315: f64, t47319: f64, t47323: f64, t47325: f64, t47327: f64, t47331: f64, t47335: f64, t47339: f64, t47343: f64, t47347: f64, t47351: f64) -> (f64, f64) {
    let t48621 = -0.44726970964441352624e-1_f64 * t26196 + t16485 - t16490 + t47293 + t47297 + t47299 + t47301 - t47303 + t47307 + t16498 - t16501;
    let t48622 = t47315 - t47319 - t47323 - t47325 + t47327 + t47331 + t47335 + t47339 + t47343 + t47347 + t47351;
    (t48621, t48622)
}
