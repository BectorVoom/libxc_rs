//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 988/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk988<F: Float>(t115432: F, t22716: F, t8631: F, t114058: F, t114061: F, t114064: F, t114073: F, t114077: F, t115420: F, t115423: F, t115428: F, t115430: F) -> F {
    let t115433 = F::cast_from(0.26044789391763585244e-1_f64) * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = F::cast_from(0.63969658155208805863e-1_f64) * t115434;
    let t115436 = t114058 + t114061 - t114064 - F::cast_from(0.82246703342411321825e-2_f64) * t115420 + F::cast_from(0.82246703342411321824e-2_f64) * t115423 - F::cast_from(0.16449340668482264365e-1_f64) * t115428 - F::cast_from(0.38381794893125283518e-1_f64) * t115430 + t115433 + t115435 - t114073 - t114077;
    t115436
}
