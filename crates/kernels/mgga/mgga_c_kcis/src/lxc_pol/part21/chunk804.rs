//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 804/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk804<F: Float>(t9531: F, t2861: F, t3221: F, t1094: F, t3168: F, t329: F, t64: F, t358: F, t283: F, t1135: F, t9528: F, t2817: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t9532 = t9531 * sigma0;
    let t9536 = t2861 * t3221;
    let t9538 = t3168 * t1094;
    let t9539 = t9538 * sigma0;
    let t9543 = t64 * t329;
    let t9545 = F::cast_from(1.0_f64) / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    let t9557 = t2861 * t2817;
    (t9532, t9536, t9538, t9539, t9545, t9546, t9552, t9557)
}
