//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 602/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk602<F: Float>(t1121: F, t829: F, t3203: F, t3202: F, t3200: F, t355: F, t283: F, sigma0: F) -> (F, F, F, F, F) {
    let t3204 = t829 * t1121;
    let t3205 = t3203 * t3204;
    let t3206 = t3202 * t3205;
    let t3207 = t3200 * t3206;
    let t3209 = t355 * sigma0;
    let t3210 = t3209 * t283;
    (t3204, t3206, t3207, t3209, t3210)
}
