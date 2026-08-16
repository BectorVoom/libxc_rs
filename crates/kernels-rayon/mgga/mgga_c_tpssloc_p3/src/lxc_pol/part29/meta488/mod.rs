//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1833;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1834;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta488(t3252: f64, t7363: f64, t7362: f64, t3248: f64, t1201: f64, t2152: f64, t24589: f64, t24760: f64, t24762: f64, t24765: f64, t24773: f64, t24778: f64, t24781: f64, t24785: f64, t24789: f64, t24792: f64, t3565: f64, t3604: f64, t470: f64, t7283: f64, t7373: f64, t7387: f64, t7389: f64, t2144: f64, t3493: f64, t1246: f64, t3620: f64, t7376: f64, t7375: f64, t23598: f64, t50: f64, t131: f64, t467: f64, t3030: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24794, t24795, t24798, t24799, t24802) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1833(t3252, t7363, t7362, t3248, t1201, t2152, t24589, t24760, t24762, t24765, t24773, t24778, t24781, t24785, t24789, t24792, t3565, t3604, t470, t7283, t7373, t7387, t7389);
        let (t24804, t24806, t24807, t24810, t24811, t24812) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1834(t2144, t3493, t1246, t3620, t7376, t7375, t23598, t50, t131, t467);
        let t24813 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1835(t3030, t461);
    (t24794, t24795, t24798, t24799, t24802, t24804, t24806, t24807, t24810, t24811, t24812, t24813)
}
