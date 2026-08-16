//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1415;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1416;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1417;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1418;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta351<F: Float>(t5107: F, t671: F, t1266: F, t4072: F, t1774: F, t2363: F, t584: F, t9212: F, t9214: F, t9216: F, t9218: F, t9220: F, t9225: F, t3951: F, t604: F, t1406: F, t2239: F, t1437: F, t2241: F, t25: F, t28: F, t4021: F, t645: F, t2307: F, t1409: F, t9321: F, t2291: F, t3966: F, zeta_threshold: F, t9330: F, t2298: F, t2244: F, t2250: F, t4007: F, t4012: F, t607: F, t634: F, t638: F, t72: F, t1410: F, t2283: F, t1426: F, t2251: F, t3997: F, t608: F, t1411: F, t1434: F, t2245: F, t2252: F, t2284: F, t2304: F, t3971: F, t3976: F, t4018: F, t629: F, t642: F, t66: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12545, t12550, t12557, t12560, t12561, t12562, t12563, t12564, t12565) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1415::<F>(t5107, t671, t1266, t4072, t1774, t2363, t584, t9212, t9214, t9216, t9218, t9220);
        let (t12566, t12568, t12571, t12582) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1416::<F>(t12560, t12561, t12562, t12563, t12564, t12565, t9225, t3951, t604, t1406, t2239, t1437, t2241);
        let (t12585, t12588, t12595, t12598, t12606) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1417::<F>(t25, t28, t4021, t645, t1437, t2307, t1409, t9321, t2291, t3966, t584, t9212, zeta_threshold);
        let t12619 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1418::<F>(t1409, t9330, t2298, t3966, t12595, t12598, t12606, t2244, t2250, t4007, t4012, t607, t634, t638);
        let t12645 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1419::<F>(t12619, t72, t1410, t2283, t1426, t2244, t2251, t3997, t608, t1411, t1434, t2245, t2252, t2284, t2304, t3971, t3976, t4018, t629, t642, t66, t80);
    (t12545, t12550, t12557, t12566, t12568, t12571, t12582, t12585, t12588, t12606, t12619, t12645)
}
