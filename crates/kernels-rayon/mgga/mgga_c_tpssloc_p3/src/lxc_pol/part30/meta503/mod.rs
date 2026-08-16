//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1820;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta503(t25548: f64, t6800: f64, t23635: f64, t1629: f64, t6743: f64, t884: f64, t4684: f64, t7619: f64, t1610: f64, t1920: f64, t1953: f64, t23633: f64, t23666: f64, t25530: f64, t25536: f64, t25541: f64, t25545: f64, t3200: f64, t4615: f64, t4669: f64, t6797: f64, t6811: f64, t6813: f64, t23384: f64, t7604: f64, t1615: f64, t6768: f64, t1060: f64, t2987: f64, t4343: f64, t4338: f64, t4509: f64, t4640: f64, t6754: f64, t1611: f64, t6764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25549, t25550, t25554, t25555, t25558, t25560) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1819(t25548, t6800, t23635, t1629, t6743, t884, t4684, t7619, t1610, t1920, t1953, t23633, t23666, t25530, t25536, t25541, t25545, t3200, t4615, t4669, t6797, t6811, t6813);
        let (t25563, t25568, t25571, t25574, t25577) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1820(t23384, t7604, t1615, t6768, t1060, t2987, t4343, t4338, t4509, t4640, t6754);
        let t25580 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1821(t1611, t6764);
    (t25549, t25550, t25554, t25555, t25558, t25560, t25563, t25568, t25571, t25574, t25577, t25580)
}
