//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1635;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1636;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1637;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta378<F: Float>(t17817: F, t2988: F, t17183: F, t4518: F, t135: F, t5844: F, t973: F, t10295: F, t10296: F, t13642: F, t13921: F, t13922: F, t13923: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t340: F, t343: F, t974: F, t5838: F, t17801: F, t17805: F, t17809: F, t17811: F, t17814: F, t2960: F, t2986: F, t5839: F, t5845: F, t17157: F, t4510: F, t17161: F, t13798: F, t17152: F, t10236: F, t5392: F, t10235: F, t13851: F, t4514: F, t10287: F, t10333: F, t10339: F, t13893: F, t13896: F, t13907: F, t13909: F, t13915: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17818, t17821, t17826, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1635::<F>(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17849, t17850, t17852) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1636::<F>(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let (t17854, t17857, t17860, t17863) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1637::<F>(t17157, t4510, t17161, t13798, t17152, t10236, t5392);
        let t17873 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1638::<F>(t10235, t17863, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t17854, t17857, t17860, t2986);
    (t17826, t17827, t17841, t17843, t17849, t17850, t17852, t17863, t17873)
}
