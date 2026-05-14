//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 701/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk701<F: Float>(t2336: F, t2675: F, t89: F, t10243: F, t10246: F, t10251: F, t10255: F, t10259: F, t10265: F, t10269: F, t10273: F, t10276: F, t10279: F, t2661: F, t9725: F, t2724: F, t811: F) -> (F, F, F, F) {
    let t10282 = t89 * t2336 * t2675;
    let t10284 = -t10243 / 9.0 - t10246 / 9.0 - t10251 / 3.0 - t10255 / 3.0 - t10259 / 18.0 - t10265 + t10269 - 5.0 / 81.0 * t10273 - t10276 / 3.0 - 2.0 / 27.0 * t10279 + t10282 / 18.0;
    let t10286 = t89 * t9725 * t2661;
    let t10292 = t2724 * t811;
    (t10282, t10284, t10286, t10292)
}
