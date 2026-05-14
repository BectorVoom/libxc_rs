//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 803/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk803<F: Float>(t230: F, t626: F, t1418: F, t1417: F, t232: F, t6054: F, t1609: F, t218: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t24286 = t626 * t230;
    let t24287 = t1418 * t24286;
    let t24289 = 0.42562405586419753087e-2 * t1417 * t24287;
    let t24307 = t6054 * t232;
    let t24310 = t1609 * sigma2;
    let t24311 = t24310 * t218;
    (t24286, t24287, t24289, t24307, t24310, t24311)
}
