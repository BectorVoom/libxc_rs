//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1016;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1017;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta265(t248: f64, t3252: f64, t3521: f64, t1227: f64, t3248: f64, t11172: f64, t1230: f64, t11163: f64, t1009: f64, t3481: f64, t1011: f64, t1212: f64, t1174: f64, t11761: f64, t11766: f64, t11770: f64, t11774: f64, t11781: f64, t11787: f64, t11792: f64, t11794: f64, t1218: f64, t3515: f64, t486: f64, t676: f64, t1216: f64, t1213: f64, t1226: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11797, t11798, t11801, t11802, t11805, t11809, t11812, t11814) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1016(t248, t3252, t3521, t1227, t3248, t11172, t1230, t11163, t1009, t3481, t1011, t1212);
        let t11817 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1017(t1174, t11761, t11766, t11770, t11774, t11781, t11787, t11792, t11794, t11798, t11802, t11805, t11809, t11814, t1218, t1227, t3515);
        let (t11818, t11820, t11821, t11825) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1018(t486, t676, t1216, t248, t1213, t1226, t3566);
    (t11797, t11801, t11805, t11809, t11812, t11814, t11817, t11818, t11820, t11821, t11825)
}
