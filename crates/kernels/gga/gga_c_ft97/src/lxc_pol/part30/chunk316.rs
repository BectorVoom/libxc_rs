//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 316/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk316<F: Float>(t2639: F, t992: F, t1212: F, t231: F, t1218: F, t1526: F, t2320: F, t2638: F, t342: F, t343: F, t281: F, t39: F, t2035: F) -> (F, F, F, F, F) {
    let t5198 = t2639 * t992;
    let t5202 = t231 * t1212;
    let t5206 = t1218 - t2638 - t1526 * t2320 * t5198 / 12.0 - t342 * t343 * t5202 / 4.0;
    let t5264 = t281 * t39;
    let t5265 = t5264 * t2035;
    (t5198, t5202, t5206, t5264, t5265)
}
