//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1833;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1834;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta488<F: Float>(t3252: F, t7363: F, t7362: F, t3248: F, t1201: F, t2152: F, t24589: F, t24760: F, t24762: F, t24765: F, t24773: F, t24778: F, t24781: F, t24785: F, t24789: F, t24792: F, t3565: F, t3604: F, t470: F, t7283: F, t7373: F, t7387: F, t7389: F, t2144: F, t3493: F, t1246: F, t3620: F, t7376: F, t7375: F, t23598: F, t50: F, t131: F, t467: F, t3030: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24794, t24795, t24798, t24799, t24802) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1833::<F>(t3252, t7363, t7362, t3248, t1201, t2152, t24589, t24760, t24762, t24765, t24773, t24778, t24781, t24785, t24789, t24792, t3565, t3604, t470, t7283, t7373, t7387, t7389);
        let (t24804, t24806, t24807, t24810, t24811, t24812) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1834::<F>(t2144, t3493, t1246, t3620, t7376, t7375, t23598, t50, t131, t467);
        let t24813 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1835::<F>(t3030, t461);
    (t24794, t24795, t24798, t24799, t24802, t24804, t24806, t24807, t24810, t24811, t24812, t24813)
}
