//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1025/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1025<F: Float>(t1978: F, t3864: F, t1983: F, t1993: F, t1996: F, t6383: F, t6384: F, t6389: F, t6396: F, t6434: F, t8469: F, t8472: F, t8476: F, t8493: F, t8497: F, t10: F, t10273: F, t18: F) -> (F, F, F, F) {
    let t10280 = t1978 * t3864;
    let t10284 = t1983 * t3864;
    let t10294 = -t1993 * t1996 * t10280 / 48.0 - t1993 * t1996 * t10284 / 48.0 - t6383 + t6384 / 96.0 + t6389 / 96.0 + t6396 / 96.0 - t8469 - t8472 - t8476 / 48.0 + t6434 / 288.0 + t8493 / 144.0 - t8497;
    let t10296 = t10273 * t10 * t18;
    (t10280, t10284, t10294, t10296)
}
