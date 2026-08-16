//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1193/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1193<F: Float>(t21833: F, t2729: F, t2731: F, t1068: F, t7539: F, t2754: F, t2814: F, t2751: F, t221: F, t2631: F, t2696: F, t1025: F, t2630: F, t7249: F) -> (F, F, F, F, F, F) {
    let t22094 = F::cast_from(0.48245938496077605201e2_f64) * t2729 * t21833 * t2731;
    let t22095 = t7539 * t1068;
    let t22102 = t2754 * t2814;
    let t22105 = F::cast_from(120.0_f64) * t2751 * t2814;
    let t22107 = t2696 * t221 * t2631;
    let t22112 = F::cast_from(0.1301229756036208781e0_f64) * t2630 * t1025 * t7249;
    (t22094, t22095, t22102, t22105, t22107, t22112)
}
