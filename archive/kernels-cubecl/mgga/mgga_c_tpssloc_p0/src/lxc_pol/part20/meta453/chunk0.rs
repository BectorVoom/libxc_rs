//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1906/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1906<F: Float>(t3359: F, t4819: F, t1136: F, t3351: F, t4823: F, t11352: F, t1682: F, t3333: F, t1155: F, t4858: F, t1695: F, t3395: F) -> (F, F, F, F, F, F, F) {
    let t15164 = t4819 * t3359;
    let t15165 = t15164 * t1136;
    let t15168 = t4823 * t3351;
    let t15171 = t1682 * t11352;
    let t15172 = t15171 * t3333;
    let t15179 = t4858 * t1155;
    let t15182 = t1695 * t3395;
    (t15164, t15165, t15168, t15171, t15172, t15179, t15182)
}
