//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk827;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk828;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk829;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta155<F: Float>(t3375: F, t440: F, t1155: F, t1156: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F, t1146: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3376, t3377) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk827::<F>(t3375, t440, t1155);
        let (t3378, t3383, t3390, t3395) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk828::<F>(t1156, t3377, t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
        let (t3396, t3399, t3400) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk829::<F>(t1156, t3395, t1146);
        let (t3401, t3402, t3403) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk830::<F>(t3400, t440, t448);
    (t3376, t3377, t3378, t3383, t3390, t3395, t3396, t3399, t3400, t3401, t3402, t3403)
}
