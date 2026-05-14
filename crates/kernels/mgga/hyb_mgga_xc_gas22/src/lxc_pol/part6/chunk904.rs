//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 904/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk904<F: Float>(t143: F, t1270: F, t1285: F, t172: F, t187: F, t2104: F, t2147: F, t3227: F, t3267: F, t740: F, t759: F, t8266: F, t8352: F, t8354: F, t8434: F, t139: F, t214: F) -> (F, F, F) {
    let t144 = 0.135e1 <= t143;
    let t8438 = piecewise3(t144, t8266 + t8352, -8.0 / 3.0 * t8354 * t187 - 16.0 / 3.0 * t3227 * t759 - 8.0 / 3.0 * t1270 * t2147 - 8.0 / 3.0 * t2104 * t1285 - 16.0 / 3.0 * t740 * t3267 - 8.0 / 3.0 * t172 * t8434);
    let t8439 = t139 * t8438;
    let t8440 = t8439 * t214;
    (t8438, t8439, t8440)
}
