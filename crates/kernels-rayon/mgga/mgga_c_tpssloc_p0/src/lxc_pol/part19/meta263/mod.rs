//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1012;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1013;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta263(t11739: f64, t1214: f64, t248: f64, t3509: f64, t3570: f64, t3506: f64, t11159: f64, t3440: f64, t11168: f64, t1177: f64, t135: f64, t3561: f64, t1174: f64, t11692: f64, t11694: f64, t11699: f64, t11703: f64, t11705: f64, t11709: f64, t11719: f64, t11724: f64, t11728: f64, t11731: f64, t11734: f64, t11738: f64, t3511: f64, t3518: f64, t11153: f64, t3439: f64, t9288: f64, t974: f64, t11147: f64, t11545: f64, t11660: f64, t1216: f64, t4582: f64, t10913: f64, t4987: f64, t3247: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11741, t11745, t11746, t11748, t11751, t11754) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1012(t11739, t1214, t248, t3509, t3570, t3506, t11159, t3440, t11168, t1177, t135, t3561);
        let t11757 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1013(t1174, t11754, t11692, t11694, t11699, t11703, t11705, t11709, t11719, t11724, t11728, t11731, t11734, t11738, t11741, t11746, t11748, t11751, t3511, t3518);
        let (t11760, t11761, t11765, t11766, t11769, t11770, t11773, t11774, t11778) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1014(t11153, t3439, t9288, t974, t11147, t11545, t11660, t1216, t4582, t10913, t4987, t3247, t415);
    (t11741, t11745, t11757, t11760, t11761, t11765, t11766, t11769, t11770, t11773, t11774, t11778)
}
