//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 748/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk748<F: Float>(t1326: F, t1330: F, t1323: F, t7761: F, t7556: F, t934: F, t2012: F, t7349: F, t270: F, t356: F, t290: F, t2010: F, t7755: F) -> (F, F, F, F, F, F, F) {
    let t35206 = t1326 * t1330;
    let t35207 = t1323 * t35206;
    let t35208 = t35207 * t7761;
    let t35210 = t934 * t7556;
    let t35212 = t7349 * t2012 * t35210;
    let t35214 = t356 * t270;
    let t35215 = t290 * t35214;
    let t35217 = t2010 * t7755 * t35215;
    (t35207, t35208, t35210, t35212, t35214, t35215, t35217)
}
