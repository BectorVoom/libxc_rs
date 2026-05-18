//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1252/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1252<F: Float>(t1524: F, t7544: F, t3639: F, t7520: F, t2676: F, t9404: F, t1112: F, t483: F, t9369: F, t2751: F, t3647: F, t3616: F, t7249: F) -> (F, F, F, F, F, F) {
    let t26010 = t7544 * t1524;
    let t26012 = t3639 * t7520;
    let t26020 = t9404 * t2676;
    let t26023 = t9369 * t483 * t1112;
    let t26038 = t2751 * t3647;
    let t26042 = t3616 * t7249;
    (t26010, t26012, t26020, t26023, t26038, t26042)
}
