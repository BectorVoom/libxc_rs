//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 804/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk804<F: Float>(t2148: F, t9296: F, t6535: F, t2139: F, t2223: F, t2614: F, t2636: F, t6241: F, t7313: F, t7608: F, t7610: F, t7618: F, t7622: F, t7627: F, t7632: F, t7925: F, t7928: F, t7939: F, t8240: F, t9280: F, t9289: F, t9294: F) -> (F,) {
    let t9297 = t2148 * t9296;
    let t9298 = t6535 * t9297;
    let t9300 = -t7608 + 0.2600466522016280569e0 * t2139 * t9280 + t7610 - 0.21341733463216935736e0 * t6241 + t7618 - t7622 + t7627 + t7632 + 0.17336443480108537126e0 * t7313 * t2636 + 0.2600466522016280569e0 * t8240 * t2614 + 0.16463622957338778997e0 * t2223 * t9289 - 0.58218257753910989057e-2 * t9294 + 0.11643651550782197811e-1 * t9298 + t7925 + t7928 + t7939;
    (t9300,)
}
