//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 881/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk881<F: Float>(t1046: F, t2729: F, t7453: F, t1048: F, t2712: F, t2723: F, t2728: F, t567: F, t222: F, t2732: F, t2702: F, t2724: F) -> (F, F, F, F, F) {
    let t7456 = F::cast_from(0.48245938496077605201e2_f64) * t2729 * t7453 * t1046;
    let t7459 = F::new(6.0) * t2712 * t1048 * t2723;
    let t7460 = t567 * t2728;
    let t7463 = F::cast_from(0.85917975471764868594e0_f64) * t222 * t7460 * t2732;
    let t7466 = F::cast_from(0.53424999999999999999e-1_f64) * t222 * t2702 * t2724;
    (t7456, t7459, t7460, t7463, t7466)
}
