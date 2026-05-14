//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 560/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk560<F: Float>(t10207: F, t10209: F, t10212: F, t10215: F, t10223: F, t10227: F, t10231: F, t1526: F, t2320: F, t2649: F, t2666: F, t2745: F, t342: F, t343: F, t3806: F, t1882: F, t2667: F) -> (F, F) {
    let t10235 = t2649 + t2745 + t10207 - t10209 / 18.0 - t10212 / 6.0 - t1526 * t3806 * t10215 / 9.0 - t1526 * t2320 * t2666 / 6.0 + t1526 * t2320 * t10223 / 6.0 - t1526 * t2320 * t10227 / 12.0 - t342 * t343 * t10231 / 4.0;
    let t10243 = t1882 * t2667;
    (t10235, t10243)
}
