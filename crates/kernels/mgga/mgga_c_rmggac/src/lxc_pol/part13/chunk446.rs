//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 446/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk446<F: Float>(t4709: F, t4746: F, t1311: F, t1314: F, t1320: F, t20: F, t253: F, t1327: F, t28: F, t1330: F, t2044: F, t1318: F, t40: F) -> (F, F, F, F, F) {
    let t4747 = t4746 * t4709;
    let t4750 = t1311 * t1314;
    let t4753 = t20 * t1320;
    let t4754 = t253 * t4753;
    let t4755 = t28 * t1327;
    let t4757 = t2044 * t4755 * t1330;
    let t4762 = t1318 * t40;
    (t4747, t4750, t4754, t4757, t4762)
}
