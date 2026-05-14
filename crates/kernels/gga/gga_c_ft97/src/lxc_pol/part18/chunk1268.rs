//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1268/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1268<F: Float>(t1286: F, t25529: F, t376: F, t25534: F, t101837: F, t102701: F, t102806: F, t102922: F, t103164: F, t103892: F, t11420: F, t11424: F, t1308: F, t1337: F, t22493: F, t22498: F, t22884: F, t22908: F, t25528: F, t26493: F, t28: F, t3000: F, t438: F) -> (F,) {
    let t104025 = 2.0 / 9.0 * t1286 * t376 * t25529;
    let t104031 = 2.0 / 9.0 * t1286 * t376 * t25534;
    let t104049 = -4.0 * t103164 - t1286 * t28 * t25528 * t22498 / 3.0 + t104025 + t1286 * t28 * t101837 * t22884 + t104031 + 4.0 * t102922 - 2.0 * t11424 * t1337 - t11420 * t1337 - 2.0 * t438 * t26493 - 2.0 * t102701 - 2.0 * t102806 - 4.0 * t103892 - 2.0 / 3.0 * t1286 * t28 * t25528 * t22493 - t1286 * t3000 * t1308 * t22908 / 9.0;
    (t104049,)
}
