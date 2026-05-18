//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 717/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk717<F: Float>(t128: F, t348: F, t107: F, t1248: F, t837: F, t899: F, t124: F, t507: F, t7190: F, t2144: F, t7262: F, t1679: F) -> (F, F, F, F, F, F, F, F) {
    let t26115 = t348 * t128;
    let t26125 = t1248 * t107;
    let t26144 = t899 * t837;
    let t26157 = t507 * t124;
    let t26283 = t507 * t7190;
    let t26287 = t899 * t2144;
    let t26291 = t507 * t7262;
    let t26346 = t1679 * t837;
    (t26115, t26125, t26144, t26157, t26283, t26287, t26291, t26346)
}
