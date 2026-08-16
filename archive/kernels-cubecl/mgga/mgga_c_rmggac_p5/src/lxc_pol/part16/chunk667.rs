//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 667/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk667<F: Float>(t884: F, t9624: F, t5888: F, t8041: F, t1356: F, t9531: F, t2474: F, t290: F) -> (F, F, F, F, F) {
    let t9625 = t884 * t9624;
    let t9627 = t8041 * t5888;
    let t9628 = t1356 * t9627;
    let t9637 = t1356 * t9531;
    let t9639 = t290 * t2474;
    (t9625, t9627, t9628, t9637, t9639)
}
