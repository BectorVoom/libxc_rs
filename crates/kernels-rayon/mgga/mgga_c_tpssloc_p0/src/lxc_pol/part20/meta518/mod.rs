//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2045;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta518(t39378: f64, t746: f64, t9720: f64, t1294: f64, t1285: f64, t9214: f64, t12451: f64, t1390: f64, t12132: f64, t588: f64, t39253: f64, t702: f64, t9453: f64, t2411: f64, t2414: f64, t701: f64, t9777: f64, t2405: f64, t2415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39568, t39570, t39571, t39577, t39581, t39585) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2044(t39378, t746, t9720, t1294, t1285, t9214, t12451, t1390, t12132, t588, t39253, t702, t9453);
        let t39590 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2045(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2046(t2405, t2415, t9453);
    (t39568, t39570, t39571, t39577, t39581, t39585, t39590, t39593)
}
