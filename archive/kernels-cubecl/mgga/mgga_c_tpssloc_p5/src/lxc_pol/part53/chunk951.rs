//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 951/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk951<F: Float>(t31632: F, t6883: F, t22724: F, t31623: F, t22716: F, t8631: F, t31631: F, t6897: F, t794: F, t113987: F, t114012: F, t114031: F) -> (F, F, F, F, F, F, F) {
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    let t115434 = t22716 * t8631;
    let t115439 = t6897 * t794 * t31631;
    let t115450 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t113987;
    let t115458 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t114012;
    let t115463 = F::cast_from(0.32298204875312312682e-2_f64) * t114031;
    (t115430, t115432, t115434, t115439, t115450, t115458, t115463)
}
