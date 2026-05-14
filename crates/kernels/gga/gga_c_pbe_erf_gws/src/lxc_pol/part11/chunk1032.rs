//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1032/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1032<F: Float>(t16485: F, t16490: F, t16498: F, t16501: F, t26196: F, t47293: F, t47297: F, t47299: F, t47301: F, t47303: F, t47307: F, t47315: F, t47319: F, t47323: F, t47325: F, t47327: F, t47331: F, t47335: F, t47339: F, t47343: F, t47347: F, t47351: F) -> (F, F) {
    let t48621 = -0.44726970964441352624e-1 * t26196 + t16485 - t16490 + t47293 + t47297 + t47299 + t47301 - t47303 + t47307 + t16498 - t16501;
    let t48622 = t47315 - t47319 - t47323 - t47325 + t47327 + t47331 + t47335 + t47339 + t47343 + t47347 + t47351;
    (t48621, t48622)
}
