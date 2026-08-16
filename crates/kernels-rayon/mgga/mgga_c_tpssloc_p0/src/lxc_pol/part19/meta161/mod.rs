//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk779;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta161(t9384: f64, t9385: f64, t2341: f64, t659: f64, t2248: f64, t9256: f64, t95: f64, t101: f64, t102: f64, t2350: f64, t662: f64, t2349: f64, t2354: f64, t103: f64, t100: f64, t2336: f64, t2343: f64, t2346: f64, t657: f64, t660: f64, t92: f64, t9374: f64, t96: f64, t109: f64, t656: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64, t9367: f64, t9371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9386, t9389, t9390, t9393, t9394, t9398, t9400, t9403) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk778(t9384, t9385, t2341, t659, t2248, t9256, t95, t101, t102, t2350, t662, t2349);
        let (t9407, t9411) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk779(t2354, t9403, t9393, t103, t100, t2336, t2343, t2346, t657, t660, t92, t9374, t9386, t9390, t9394, t9400, t96);
        let (t9412, t9416) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk780(t109, t656, t9411, t64, t9358, t9359, t9361, t9363, t9367, t9371);
    (t9386, t9389, t9390, t9393, t9394, t9398, t9403, t9407, t9411, t9412, t9416)
}
