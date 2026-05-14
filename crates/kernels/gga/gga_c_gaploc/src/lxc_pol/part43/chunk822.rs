//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 822/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk822<F: Float>(t12032: F, t2497: F, t12148: F, t1382: F, t921: F, t13838: F, t5559: F, t841: F, t12270: F, t1960: F, t977: F, t2595: F, t38892: F, t12272: F, t7324: F, t3749: F, t7822: F) -> (F, F, F, F, F, F, F) {
    let t47075 = t12032 * t2497;
    let t47077 = t1382 * t12148 * t921;
    let t47080 = t5559 * t13838 * t841;
    let t47083 = t1960 * t12270 * t977;
    let t47085 = t38892 * t2595;
    let t47087 = t7324 * t12272;
    let t47096 = t7822 * t3749;
    (t47075, t47077, t47080, t47083, t47085, t47087, t47096)
}
