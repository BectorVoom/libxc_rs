//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 689/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk689<F: Float>(t1180: F, t673: F, t1182: F, t128: F, t118: F, t1184: F, t1986: F, t7487: F, t7757: F, t1326: F, t1330: F, t1323: F, t7761: F, t7556: F, t934: F, t2012: F, t7349: F) -> (F, F, F, F, F, F, F, F) {
    let t35190 = t1180 * t673;
    let t35192 = t128 * t1182;
    let t35195 = t1986 * t118 * t35192 * t1184;
    let t35204 = t7487 * t7757;
    let t35206 = t1326 * t1330;
    let t35207 = t1323 * t35206;
    let t35208 = t35207 * t7761;
    let t35210 = t934 * t7556;
    let t35212 = t7349 * t2012 * t35210;
    (t35190, t35192, t35195, t35204, t35207, t35208, t35210, t35212)
}
