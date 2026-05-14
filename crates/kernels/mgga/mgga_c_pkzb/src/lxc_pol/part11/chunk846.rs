//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 846/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk846<F: Float>(t3702: F, t6065: F, t2156: F, t3698: F, t1306: F, t803: F, t9336: F, t9338: F, t9345: F, t9347: F, t9350: F, t9354: F, t9358: F, t9361: F, t9363: F, t9365: F, t9367: F, t9392: F, t9394: F, t9396: F, t9400: F, t9530: F, t9535: F, t9537: F) -> (F, F, F) {
    let t9721 = t3702 * t6065;
    let t9725 = t3698 * t2156;
    let t9728 = 2.0 * t1306 * t803 * t9721 - t1306 * t803 * t9725 + t9336 + t9338 - t9345 - t9347 + t9350 - t9354 + t9358 - t9361 + t9363 - t9365 + t9367 + t9392 + t9394 - t9396 + t9400 + t9530 - t9535 + t9537;
    (t9721, t9725, t9728)
}
