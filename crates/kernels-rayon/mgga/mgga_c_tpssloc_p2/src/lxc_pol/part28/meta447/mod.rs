//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1633;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1634;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1635;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta447(t23917: f64, t510: f64, t1266: f64, t7056: f64, t671: f64, t7156: f64, t111: f64, t7039: f64, t2035: f64, t2319: f64, t2095: f64, t22578: f64, t22584: f64, t7170: f64, t12734: f64, t1393: f64, t1983: f64, t2036: f64, t2040: f64, t2079: f64, t2314: f64, t2323: f64, t2364: f64, t23909: f64, t3652: f64, t3929: f64, t4034: f64, t652: f64, t672: f64, t7040: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7166: f64, t9348: f64, t2094: f64, t531: f64, t22596: f64, t7025: f64, t9239: f64, t33: f64, t625: f64, t2240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23918, t23929, t23933, t23938) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1633(t23917, t510, t1266, t7056, t671, t7156, t111, t7039);
        let (t23941, t23951, t23953, t23956) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1634(t2035, t2319, t2095, t22578, t22584, t7170, t1266, t12734, t1393, t1983, t2036, t2040, t2079, t2314, t2323, t2364, t23909, t23918, t23929, t23933, t23938, t3652, t3929, t4034, t510, t652, t672, t7040, t7042, t7050, t7057, t7061, t7166, t9348);
        let (t23958, t23963) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1635(t2094, t531, t22596, t7025, t9239);
        let (t23966, t23967) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1636(t33, t625, t2240);
    (t23918, t23929, t23933, t23938, t23941, t23951, t23953, t23956, t23958, t23963, t23966, t23967)
}
