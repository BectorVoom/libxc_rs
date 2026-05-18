//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1109/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1109<F: Float>(t2195: F, t2238: F, t338: F, t237: F, t6131: F, t3185: F, t6418: F, t926: F, t2411: F, t54: F, t2380: F, t6368: F) -> (F, F, F, F, F) {
    let t18617 = t338 / t2238 / t2195;
    let t18627 = t237 * t6131;
    let t18655 = t3185 * t926 * t6418;
    let t18657 = t54 * t2411;
    let t18659 = t2380 * t18657 * t6368;
    (t18617, t18627, t18655, t18657, t18659)
}
