//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 448/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk448<F: Float>(t2033: F, t675: F, t2002: F, t688: F, t708: F, t140: F, t1885: F, t35: F, t1890: F, t704: F, t137: F, t697: F) -> (F, F, F, F, F, F) {
    let t2034 = t2033 * t675;
    let t2038 = t688 * t2002;
    let t2042 = t708 * t708;
    let t2047 = F::new(2.0) / F::new(81.0) * t35 * t1885 * t140;
    let t2048 = t1890 * t704;
    let t2051 = F::new(1.0) / t697 / t137;
    (t2034, t2038, t2042, t2047, t2048, t2051)
}
