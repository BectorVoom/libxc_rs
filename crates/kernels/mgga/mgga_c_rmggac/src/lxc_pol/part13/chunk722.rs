//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 722/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk722<F: Float>(t2144: F, t899: F, t507: F, t7262: F, t1679: F, t837: F, t325: F, t5011: F, t117: F, t1249: F, t4968: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t26287 = t899 * t2144;
    let t26291 = t507 * t7262;
    let t26346 = t1679 * t837;
    let t26370 = t5011 * t325;
    let t26387 = t1249 * t117;
    let t26490 = t4968 * t325;
    let t26531 = t794 * t325;
    (t26287, t26291, t26346, t26370, t26387, t26490, t26531)
}
