//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 733/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk733<F: Float>(t507: F, t7190: F, t2144: F, t899: F, t7262: F, t1679: F, t837: F, t325: F, t5011: F, t5058: F, t128: F, t25640: F) -> (F, F, F, F, F, F, F) {
    let t26283 = t507 * t7190;
    let t26287 = t899 * t2144;
    let t26291 = t507 * t7262;
    let t26346 = t1679 * t837;
    let t26370 = t5011 * t325;
    let t26857 = t5058 * t325;
    let t27041 = t25640 * t128;
    (t26283, t26287, t26291, t26346, t26370, t26857, t27041)
}
