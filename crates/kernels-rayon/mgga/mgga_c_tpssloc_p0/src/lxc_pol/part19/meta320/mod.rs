//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1134;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1136;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta320(t39378: f64, t746: f64, t9720: f64, t1294: f64, t1285: f64, t9214: f64, t12451: f64, t1390: f64, t12132: f64, t588: f64, t39253: f64, t702: f64, t9453: f64, t12012: f64, t12156: f64, t12477: f64, t1307: f64, t1388: f64, t193: f64, t3719: f64, t3918: f64, t39529: f64, t39531: f64, t39533: f64, t39539: f64, t39541: f64, t39549: f64, t39563: f64, t5126: f64, t571: f64, t2411: f64, t2414: f64, t701: f64, t9777: f64, t2405: f64, t2415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39568, t39570, t39572, t39577, t39582, t39585) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1134(t39378, t746, t9720, t1294, t1285, t9214, t12451, t1390, t12132, t588, t39253, t702, t9453);
        let t39586 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135(t12012, t12156, t12477, t1307, t1388, t1390, t193, t3719, t3918, t39529, t39531, t39533, t39539, t39541, t39549, t39563, t39570, t39572, t39577, t39582, t39585, t5126, t571);
        let t39590 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1136(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1137(t2405, t2415, t9453);
    (t39568, t39570, t39572, t39582, t39585, t39586, t39590, t39593)
}
