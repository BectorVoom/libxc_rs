//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 427/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk427<F: Float>(t1611: F, t240: F, t2727: F, t2733: F, t2744: F, t2748: F, t555: F, t172: F, t32: F, t5: F, t142: F, t814: F, t298: F, t831: F, t28: F, t813: F) -> (F, F, F, F, F) {
    let t2752 = t2727 - t2733 + t240 * (-t1611 * t2748 + t2744 * t555 - t2727 + t2733);
    let t2849 = 0.14764770444444444444e-2 * t5 * t172 * t32;
    let t2850 = t142 * t814;
    let t2853 = 0.35616666666666666667e-1 * t298 * t2850 * t831;
    let t2854 = t813 * t28;
    (t2752, t2849, t2850, t2853, t2854)
}
