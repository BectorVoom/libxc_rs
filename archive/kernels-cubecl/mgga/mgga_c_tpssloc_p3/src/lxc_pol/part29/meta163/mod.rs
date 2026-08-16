//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk869;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk870;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta163<F: Float>(t3545: F, t456: F, t1197: F, t135: F, t1174: F, t1196: F, t2250: F, t974: F, t1176: F, t3247: F, t2244: F, t3242: F, t3439: F, t225: F, t3481: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3547, t3548, t3549, t3551, t3552, t3556, t3557, t3560) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk869::<F>(t3545, t456, t1197, t135, t1174, t1196, t2250, t974, t1176, t3247, t2244, t3242, t3439);
        let (t3561, t3562, t3565) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk870::<F>(t2244, t3560, t974, t225, t3481);
        let t3566 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk871::<F>(t3565, t68);
    (t3547, t3548, t3549, t3551, t3552, t3556, t3557, t3561, t3562, t3565, t3566)
}
