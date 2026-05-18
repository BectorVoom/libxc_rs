//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1347/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1347<F: Float>(t2967: F, t31767: F, t2822: F, t3832: F, t7063: F, t10529: F, t8613: F, t24915: F, t3568: F, t12062: F, t4908: F, t12277: F, t2469: F, t36119: F, t36122: F, t36266: F, t36269: F, t36270: F, t36271: F, t36272: F, t36275: F, t36280: F, t36283: F, t36285: F, t36288: F, t972: F) -> (F, F, F, F) {
    let t36290 = F::new(4.0) * t31767 * t2967;
    let t36293 = F::new(6.0) * t7063 * t3832 * t2822;
    let t36295 = F::new(4.0) * t10529 * t8613;
    let t36297 = F::new(4.0) * t24915 * t3568;
    let t36299 = F::new(4.0) * t4908 * t12062;
    let t36300 = F::new(4.0) * t12277 * t2469 * t972 - t36119 - t36122 + t36266 - t36269 + t36270 + t36271 - t36272 + t36275 + t36280 - t36283 + t36285 - t36288 - t36290 - t36293 - t36295 + t36297 - t36299;
    (t36290, t36295, t36299, t36300)
}
