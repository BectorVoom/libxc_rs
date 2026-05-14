//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 823/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk823<F: Float>(t1897: F, t8232: F, t1882: F, t8362: F, t1868: F, t37292: F, t37254: F, t37257: F, t37261: F, t37266: F, t37271: F, t37275: F, t37277: F, t37281: F, t37285: F, t37289: F, t37296: F, t37300: F, t38397: F, t38400: F) -> (F, F, F, F) {
    let t38746 = t8232 * t1897;
    let t38748 = t1882 * t8362;
    let t38759 = t8232 * t1868;
    let t38771 = 280.0 / 243.0 * t37292;
    let t38776 = 8.0 / 9.0 * t37254 - 8.0 / 3.0 * t37257 + 4.0 / 9.0 * t37261 + 8.0 / 9.0 * t37266 - 8.0 / 27.0 * t37271 + 8.0 / 3.0 * t37275 + 8.0 / 9.0 * t37277 + 4.0 / 9.0 * t37281 + 2.0 / 3.0 * t37285 + 8.0 / 3.0 * t37289 + t38771 - 8.0 / 3.0 * t37296 - 8.0 / 3.0 * t37300 - t38397 / 3.0 + 3.0 / 4.0 * t38400;
    (t38746, t38748, t38759, t38776)
}
