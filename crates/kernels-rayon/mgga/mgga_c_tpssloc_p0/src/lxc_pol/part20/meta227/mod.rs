//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1307;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta227(t9256: f64, t95: f64, t101: f64, t102: f64, t2350: f64, t662: f64, t2349: f64, t2354: f64, t103: f64, t100: f64, t2336: f64, t2343: f64, t2346: f64, t657: f64, t660: f64, t92: f64, t9374: f64, t9386: f64, t9390: f64, t96: f64, t109: f64, t656: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64, t9367: f64, t9371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9393, t9398, t9399, t9400, t9404, t9407, t9408, t9411) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1307(t9256, t95, t101, t102, t2350, t662, t2349, t2354, t103, t100, t2336, t2343, t2346, t657, t660, t92, t9374, t9386, t9390, t96);
        let (t9412, t9416) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1308(t109, t656, t9411, t64, t9358, t9359, t9361, t9363, t9367, t9371);
    (t9393, t9398, t9399, t9400, t9404, t9407, t9408, t9411, t9412, t9416)
}
