//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1060/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1060<F: Float>(t134: F, t966: F, t8133: F, t7938: F, t8676: F, t26995: F, t7200: F, t7453: F, t1045: F, t818: F, t332: F, t7877: F) -> (F, F, F, F, F, F, F) {
    let t28191 = t966 * t134;
    let t28192 = t28191 * t8133;
    let t28254 = t8676 * t7938;
    let t28346 = t26995 * t7200;
    let t28353 = t26995 * t7453;
    let t28370 = t1045 * t818;
    let t28415 = t332 * t134;
    let t28416 = t28415 * t7877;
    (t28192, t28254, t28346, t28353, t28370, t28415, t28416)
}
