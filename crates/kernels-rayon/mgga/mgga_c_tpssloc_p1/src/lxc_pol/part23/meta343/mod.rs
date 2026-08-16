//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta343(t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39284: f64, t39289: f64, t39291: f64, t39293: f64, t39295: f64, t39298: f64, t683: f64, t702: f64, t39378: f64, t746: f64, t9720: f64, t1294: f64, t12132: f64, t588: f64, t39253: f64, t9453: f64, t2411: f64, t2414: f64, t701: f64, t9777: f64, t2405: f64, t2415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t39563 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126(t39273, t39275, t39278, t39281, t39284, t39289, t39291, t39293, t39295, t39298, t683, t702);
        let (t39568, t39570, t39582, t39585) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1127(t39378, t746, t9720, t1294, t12132, t588, t39253, t702, t9453);
        let t39590 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1128(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1129(t2405, t2415, t9453);
    (t39563, t39568, t39570, t39582, t39585, t39590, t39593)
}
