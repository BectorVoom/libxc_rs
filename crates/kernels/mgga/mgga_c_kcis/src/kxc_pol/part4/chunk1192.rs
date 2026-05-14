//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1192/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1192<F: Float>(t4298: F, t6002: F, t16609: F, t556: F, t572: F, t1533: F, t16721: F, t4261: F, t6027: F, t2042: F, t4273: F, t571: F, t4265: F, t1543: F, t5935: F, t2061: F, t4297: F) -> (F, F, F, F, F, F, F) {
    let t17484 = t6002 * t4298;
    let t17486 = t556 * t16609;
    let t17487 = t572 * t17486;
    let t17488 = t1533 * t17487;
    let t17490 = t4261 * t16721;
    let t17491 = t6027 * t17490;
    let t17493 = t2042 * t4273;
    let t17494 = t571 * t17493;
    let t17496 = t2042 * t4265;
    let t17497 = t1533 * t17496;
    let t17499 = t1543 * t5935;
    let t17501 = t2061 * t4297;
    (t17484, t17488, t17491, t17494, t17497, t17499, t17501)
}
