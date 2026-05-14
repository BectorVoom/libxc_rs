//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1211/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1211<F: Float>(t10295: F, t1890: F, t10290: F, t10299: F, t7942: F, t10288: F, t20278: F, t2028: F, t20280: F, t20282: F, t20355: F, t24131: F, t24133: F, t24135: F, t24137: F, t3171: F, t572: F) -> (F,) {
    let t28228 = t1890 * t10295;
    let t28230 = t1890 * t10290;
    let t28232 = t7942 * t10299;
    let t28242 = -4.0 / 9.0 * t572 * t3171 * t10288 * t2028 - 4.0 / 81.0 * t28228 + 10.0 / 729.0 * t28230 - 44.0 / 243.0 * t28232 + 4.0 / 243.0 * t20355 + 28.0 / 729.0 * t20278 - 2.0 / 243.0 * t20280 - 4.0 / 729.0 * t20282 + 16.0 / 243.0 * t24131 - 16.0 / 729.0 * t24133 + 2.0 / 243.0 * t24135 + 10.0 / 729.0 * t24137;
    (t28242,)
}
