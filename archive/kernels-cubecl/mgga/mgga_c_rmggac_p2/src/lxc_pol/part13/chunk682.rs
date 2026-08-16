//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 682/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk682<F: Float>(t117: F, t6477: F, t2295: F, t7204: F, t8902: F, t7192: F, t8906: F, t5888: F, t875: F, t1971: F, t3351: F, t2310: F, t7720: F) -> (F, F, F, F, F, F, F) {
    let t9128 = t6477 * t117;
    let t9129 = t9128 * t2295;
    let t9133 = t7204 * t8902;
    let t9135 = t7192 * t8906;
    let t9137 = t875 * t5888;
    let t9138 = t1971 * t9137;
    let t9139 = t3351 * t9138;
    let t9143 = t7720 * t2310;
    (t9128, t9129, t9133, t9135, t9138, t9139, t9143)
}
