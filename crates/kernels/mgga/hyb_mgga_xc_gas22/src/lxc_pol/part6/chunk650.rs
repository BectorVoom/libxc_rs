//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 650/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk650<F: Float>(t181: F, t740: F, t178: F, t1270: F, t173: F, t180: F, t3227: F, t3232: F, t3245: F, t3246: F, t3252: F, t747: F, t751: F, t1282: F, t172: F, t184: F, t2116: F, t3231: F, t3235: F, t742: F, t756: F) -> (F, F, F, F) {
    let t3255 = t740 * t181;
    let t3258 = t178 * t740;
    let t3264 = -2.0 * t3245 * t3246 + t747 * t3227 * t180 / 2.0 + t3252 * t3246 / 4.0 - 4.0 * t3255 * t1270 - t3258 * t3232 - 4.0 * t751 * t3227 - t173 * t3227 * t180;
    let t3267 = -t3231 * t3232 / 2.0 + 2.0 * t2116 * t3235 - t742 * t3227 + 2.0 * t3227 * t184 + 2.0 * t1270 * t756 + 2.0 * t740 * t1282 + 2.0 * t172 * t3264;
    (t3255, t3258, t3264, t3267)
}
