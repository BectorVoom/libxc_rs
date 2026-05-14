//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 758/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk758<F: Float>(t37292: F, t7788: F, t8494: F, t1564: F, t446: F, t379: F, t8544: F, t7824: F, t37254: F, t37257: F, t37261: F, t37266: F, t37271: F, t37275: F, t37277: F, t37281: F, t37285: F, t37289: F) -> (F, F, F, F, F) {
    let t37293 = 140.0 / 243.0 * t37292;
    let t37294 = t7788 * t8494;
    let t37296 = t446 * t1564 * t37294;
    let t37298 = t8544 * t379;
    let t37300 = t446 * t7824 * t37298;
    let t37302 = 4.0 / 9.0 * t37254 - 4.0 / 3.0 * t37257 + 2.0 / 9.0 * t37261 + 4.0 / 9.0 * t37266 - 4.0 / 27.0 * t37271 + 4.0 / 3.0 * t37275 + 4.0 / 9.0 * t37277 + 2.0 / 9.0 * t37281 + t37285 / 3.0 + 4.0 / 3.0 * t37289 + t37293 - 4.0 / 3.0 * t37296 - 4.0 / 3.0 * t37300;
    (t37294, t37296, t37298, t37300, t37302)
}
