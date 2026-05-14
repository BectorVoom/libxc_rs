//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 810/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk810<F: Float>(t37292: F, t1766: F, t473: F, t8336: F, t91: F, t1767: F, t1808: F, t8345: F, t37254: F, t37257: F, t37261: F, t37266: F, t37271: F, t37275: F, t37277: F, t37281: F, t37285: F, t37289: F, t37296: F, t37300: F) -> (F, F, F) {
    let t38392 = 280.0 / 81.0 * t37292;
    let t38397 = t91 * t1766 * t8336 * t473;
    let t38400 = t91 * t8345 * t1767 * t1808;
    let t38402 = 8.0 / 3.0 * t37254 - 8.0 * t37257 + 4.0 / 3.0 * t37261 + 8.0 / 3.0 * t37266 - 8.0 / 9.0 * t37271 + 8.0 * t37275 + 8.0 / 3.0 * t37277 + 4.0 / 3.0 * t37281 + 2.0 * t37285 + 8.0 * t37289 + t38392 - 8.0 * t37296 - 8.0 * t37300 - t38397 + 9.0 / 4.0 * t38400;
    (t38397, t38400, t38402)
}
