//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 663/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk663<F: Float>(t797: F, t874: F, t338: F, t3814: F, t837: F, t892: F, t1318: F, t21: F, t41: F, t1342: F, t1249: F, t325: F, t128: F, t348: F, t107: F, t1248: F) -> (F, F, F, F, F, F, F, F) {
    let t25854 = t797 * t874;
    let t25877 = t3814 * t338;
    let t25918 = t892 * t837;
    let t26004 = t1318 * t1318;
    let t26007 = t21 / t41 / t26004;
    let t26077 = t1342 * t1342;
    let t26078 = 1.0 / t26077;
    let t26093 = t1249 * t325;
    let t26115 = t348 * t128;
    let t26125 = t1248 * t107;
    (t25854, t25877, t25918, t26007, t26078, t26093, t26115, t26125)
}
