//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1044/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1044<F: Float>(t103: F, t332: F, t875: F, t9739: F, t147: F, t19: F, t2315: F, t3295: F, t966: F, t2902: F, t760: F, t786: F, t9740: F) -> (F, F, F, F) {
    let t24086 = t9739 * t332 * t103 * t875;
    let t24092 = t3295 * t966 * t2315 * t19 * t147;
    let t24095 = t2902 * t760;
    let t24110 = t9740 * t103 * t786;
    (t24086, t24092, t24095, t24110)
}
