//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1182/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1182<F: Float>(t17349: F, t4292: F, t2061: F, t4287: F, t4286: F, t4266: F, t6016: F, t16665: F, t6028: F, t6027: F, t12505: F, t2039: F, t2062: F, t4278: F, t12568: F, t5919: F) -> (F, F, F, F, F, F, F) {
    let t17350 = t4292 * t17349;
    let t17352 = t2061 * t4287;
    let t17353 = t4286 * t17352;
    let t17355 = t6016 * t4266;
    let t17357 = t6028 * t16665;
    let t17358 = t6027 * t17357;
    let t17360 = t12505 * t2039;
    let t17362 = t4278 * t2062;
    let t17364 = t12568 * t5919;
    (t17350, t17353, t17355, t17358, t17360, t17362, t17364)
}
