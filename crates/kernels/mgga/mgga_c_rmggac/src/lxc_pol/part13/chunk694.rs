//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 694/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk694<F: Float>(t903: F, t9399: F, t1635: F, t699: F, t1364: F, t2211: F, t5898: F, t884: F, t1562: F, t2265: F, t2466: F, t4041: F) -> (F, F, F, F, F, F, F) {
    let t9400 = t903 * t9399;
    let t9402 = t699 * t1635;
    let t9403 = t1364 * t9402;
    let t9405 = t2211 * t5898;
    let t9406 = t884 * t9405;
    let t9408 = t1562 * t2265;
    let t9410 = t4041 * t2466;
    (t9400, t9402, t9403, t9405, t9406, t9408, t9410)
}
