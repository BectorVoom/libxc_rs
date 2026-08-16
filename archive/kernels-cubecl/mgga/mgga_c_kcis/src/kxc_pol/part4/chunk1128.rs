//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1128/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1128<F: Float>(t1154: F, t330: F, t348: F, t1758: F, t3251: F, t1114: F, t13786: F, t345: F, t2952: F, t4601: F, t4600: F, t313: F, t4625: F) -> (F, F, F, F, F, F, F) {
    let t14269 = t1154 * t348 * t330;
    let t14272 = t3251 * t1758;
    let t14274 = t1114 * t13786;
    let t14275 = t345 * t14274;
    let t14278 = t4601 * t2952;
    let t14279 = t4600 * t14278;
    let t14282 = t313 * t4625;
    (t14269, t14272, t14274, t14275, t14278, t14279, t14282)
}
