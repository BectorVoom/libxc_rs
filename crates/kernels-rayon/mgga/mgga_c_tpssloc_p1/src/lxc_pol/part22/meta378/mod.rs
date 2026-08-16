//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1635;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1636;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1637;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta378(t17817: f64, t2988: f64, t17183: f64, t4518: f64, t135: f64, t5844: f64, t973: f64, t10295: f64, t10296: f64, t13642: f64, t13921: f64, t13922: f64, t13923: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t340: f64, t343: f64, t974: f64, t5838: f64, t17801: f64, t17805: f64, t17809: f64, t17811: f64, t17814: f64, t2960: f64, t2986: f64, t5839: f64, t5845: f64, t17157: f64, t4510: f64, t17161: f64, t13798: f64, t17152: f64, t10236: f64, t5392: f64, t10235: f64, t13851: f64, t4514: f64, t10287: f64, t10333: f64, t10339: f64, t13893: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17818, t17821, t17826, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1635(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17849, t17850, t17852) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1636(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let (t17854, t17857, t17860, t17863) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1637(t17157, t4510, t17161, t13798, t17152, t10236, t5392);
        let t17873 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1638(t10235, t17863, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t17854, t17857, t17860, t2986);
    (t17826, t17827, t17841, t17843, t17849, t17850, t17852, t17863, t17873)
}
