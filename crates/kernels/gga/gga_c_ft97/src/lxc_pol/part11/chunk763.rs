//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 763/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk763<F: Float>(t1581: F, t7773: F, t89: F, t1554: F, t1636: F, t1560: F, t37303: F, t37308: F, t37313: F, t37317: F, t37322: F, t37326: F, t37328: F, t37330: F, t37332: F, t37335: F, t37336: F, t37340: F) -> (F, F, F, F) {
    let t37343 = t89 * t7773 * t1581;
    let t37344 = 4.0 / 27.0 * t37343;
    let t37345 = t1636 * t1554;
    let t37347 = t89 * t37345 * t1560;
    let t37348 = 8.0 / 81.0 * t37347;
    let t37349 = 4.0 / 9.0 * t37303 + 20.0 / 81.0 * t37308 - 10.0 / 27.0 * t37313 - 2.0 * t37317 + 4.0 / 3.0 * t37322 + 2.0 / 9.0 * t37326 - 4.0 / 9.0 * t37328 + 4.0 / 27.0 * t37330 - 4.0 / 27.0 * t37332 + t37335 - 2.0 / 9.0 * t37336 + 4.0 / 3.0 * t37340 - t37344 - t37348;
    (t37343, t37345, t37347, t37349)
}
