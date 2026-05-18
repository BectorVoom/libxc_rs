//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 446/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk446<F: Float>(t126: F, t2435: F, t824: F, t190: F, t291: F, t329: F, t442: F, t891: F) -> (F, F, F, F, F) {
    let t2436 = t2435 * t126;
    let t2437 = t824 * t2436;
    let t2438 = t190 * t291;
    let t2439 = t2438 * t329;
    let t2440 = t891 * t442;
    (t2436, t2437, t2438, t2439, t2440)
}
