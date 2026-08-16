//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2174;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta522<F: Float>(t17817: F, t2988: F, t17183: F, t4518: F, t135: F, t5844: F, t973: F, t10295: F, t10296: F, t13642: F, t13921: F, t13922: F, t13923: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t340: F, t343: F, t974: F, t5838: F, t17801: F, t17805: F, t17809: F, t17811: F, t17814: F, t2960: F, t2986: F, t5839: F, t5845: F) -> (F, F, F, F, F, F, F, F) {
        let (t17818, t17821, t17826, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2174::<F>(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17844, t17849, t17852) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2175::<F>(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
    (t17818, t17821, t17826, t17841, t17843, t17844, t17849, t17852)
}
