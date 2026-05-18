//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 744/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk744<F: Float>(t2160: F, t638: F, t7216: F, t7220: F, t2186: F, t7914: F, t1289: F, t2039: F, t270: F, t2046: F, t2050: F, t31: F) -> (F, F, F, F, F) {
    let t35053 = t638 * t2160 * t7216;
    let t35056 = t638 * t2160 * t7220;
    let t35058 = t2186 * t7914;
    let t35106 = t638 * t2039 * t1289 * t270;
    let t35110 = t2046 * t2050 * t1289 * t31;
    (t35053, t35056, t35058, t35106, t35110)
}
