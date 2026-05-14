//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 455/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk455<F: Float>(t1842: F, t381: F, t385: F, t5404: F, t4342: F, t5420: F, t4352: F, t4214: F, t4220: F, t4232: F, t4252: F, t4255: F, t4259: F, t4338: F, t4351: F, t5407: F, t5409: F) -> (F, F, F, F, F, F, F) {
    let t5997 = t381 * t1842;
    let t5998 = 4.0 * t5997;
    let t5999 = t385 * t1842;
    let t6000 = 4.0 * t5999;
    let t6001 = 16.0 * t5404;
    let t6002 = 0.11696447245269292414e1 * t4342;
    let t6003 = 2.0 * t5420;
    let t6004 = 0.24415263074675393405e-3 * t4352;
    let t6005 = -t4338 + t5998 - t6000 + t4214 - t4220 - t6001 - t5407 - t5409 + t6002 + t6003 + t4232 + t4252 - t4255 - t4259 - t4351 + t6004;
    (t5998, t6000, t6001, t6002, t6003, t6004, t6005)
}
