//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1127/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1127<F: Float>(t41: F, t85: F, t8565: F, t4589: F, t1109: F, t13744: F, t345: F, t1098: F, t4672: F, t1670: F, t3288: F, t3270: F) -> (F, F, F, F, F, F) {
    let t14249 = t85 * t8565 * t41;
    let t14250 = t14249 * t4589;
    let t14252 = t1109 * t13744;
    let t14253 = t345 * t14252;
    let t14260 = F::cast_from(0.13140859333333333333e-2_f64) * t1098 * t4672;
    let t14262 = t3288 * t1670;
    let t14263 = t14262 * t3270;
    (t14249, t14250, t14252, t14253, t14260, t14263)
}
