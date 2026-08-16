//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1425;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1426;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1427;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1428;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta344(t5107: f64, t671: f64, t1266: f64, t4072: f64, t1774: f64, t2363: f64, t584: f64, t9212: f64, t9214: f64, t9216: f64, t9218: f64, t9220: f64, t9225: f64, t3951: f64, t604: f64, t1406: f64, t2239: f64, t1437: f64, t2241: f64, t25: f64, t28: f64, t4021: f64, t645: f64, t2307: f64, t1409: f64, t9321: f64, t2291: f64, t3966: f64, zeta_threshold: f64, t9330: f64, t2298: f64, t2244: f64, t2250: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64, t72: f64, t1410: f64, t2283: f64, t1426: f64, t2251: f64, t3997: f64, t608: f64, t1411: f64, t1434: f64, t2245: f64, t2252: f64, t2284: f64, t2304: f64, t3971: f64, t3976: f64, t4018: f64, t629: f64, t642: f64, t66: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12545, t12550, t12557, t12560, t12561, t12562, t12563, t12564, t12565) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1425(t5107, t671, t1266, t4072, t1774, t2363, t584, t9212, t9214, t9216, t9218, t9220);
        let (t12566, t12568, t12571, t12582) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1426(t12560, t12561, t12562, t12563, t12564, t12565, t9225, t3951, t604, t1406, t2239, t1437, t2241);
        let (t12585, t12588, t12595, t12598, t12606) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1427(t25, t28, t4021, t645, t1437, t2307, t1409, t9321, t2291, t3966, t584, t9212, zeta_threshold);
        let t12619 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1428(t1409, t9330, t2298, t3966, t12595, t12598, t12606, t2244, t2250, t4007, t4012, t607, t634, t638);
        let t12645 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1429(t12619, t72, t1410, t2283, t1426, t2244, t2251, t3997, t608, t1411, t1434, t2245, t2252, t2284, t2304, t3971, t3976, t4018, t629, t642, t66, t80);
    (t12545, t12550, t12557, t12566, t12568, t12571, t12582, t12585, t12588, t12606, t12619, t12645)
}
