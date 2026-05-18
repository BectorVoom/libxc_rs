//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1322/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1322<F: Float>(t13161: F, t2842: F, t7718: F, t13133: F, t9370: F, t13200: F, t42530: F, t1020: F, t4792: F, t92917: F, t13256: F, t26760: F) -> (F, F, F, F, F) {
    let t96318 = t2842 * t7718 * t13161;
    let t96321 = t9370 * t7718 * t13133;
    let t96324 = t42530 * t7718 * t13200;
    let t96327 = t1020 * t92917 * t4792;
    let t96330 = t1020 * t26760 * t13256;
    (t96318, t96321, t96324, t96327, t96330)
}
