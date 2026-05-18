//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 544/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk544<F: Float>(t2873: F, t2874: F, t730: F, t1066: F, t154: F, t2048: F, t276: F, t153: F, t275: F) -> (F, F, F, F, F) {
    let t2875 = t2873 * t2874;
    let t2877 = F::new(0.17315859105681463759e2) * t730 * t2875;
    let t2883 = t154 * t2048 * t1066;
    let t2884 = t276 * t2883;
    let t2886 = t275 * t153;
    (t2875, t2877, t2883, t2884, t2886)
}
