//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2174;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta522(t17817: f64, t2988: f64, t17183: f64, t4518: f64, t135: f64, t5844: f64, t973: f64, t10295: f64, t10296: f64, t13642: f64, t13921: f64, t13922: f64, t13923: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t340: f64, t343: f64, t974: f64, t5838: f64, t17801: f64, t17805: f64, t17809: f64, t17811: f64, t17814: f64, t2960: f64, t2986: f64, t5839: f64, t5845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17818, t17821, t17826, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2174(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17844, t17849, t17852) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2175(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
    (t17818, t17821, t17826, t17841, t17843, t17844, t17849, t17852)
}
