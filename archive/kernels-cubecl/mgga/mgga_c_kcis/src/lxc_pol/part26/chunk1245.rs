//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1245/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1245<F: Float>(t11825: F, t491: F, t1386: F, t16962: F, t28513: F, t4142: F, t16836: F, t3717: F, t28347: F, t94246: F, t27369: F, t28505: F, t3728: F) -> (F, F, F, F, F, F, F) {
    let t98310 = t11825 * t491;
    let t98315 = t16962 * t1386;
    let t98344 = t4142 * t28513;
    let t98359 = t16836 * t3717;
    let t98364 = t94246 * t28347;
    let t98365 = t27369 * t98364;
    let t98380 = t3728 * t28505;
    (t98310, t98315, t98344, t98359, t98364, t98365, t98380)
}
