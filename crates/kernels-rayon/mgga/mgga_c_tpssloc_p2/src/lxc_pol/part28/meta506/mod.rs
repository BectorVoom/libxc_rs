//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1750;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1751;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1752;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1753;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta506(t111: f64, t7786: f64, t1268: f64, t12725: f64, t1458: f64, t19456: f64, t2039: f64, t2314: f64, t23938: f64, t26114: f64, t26117: f64, t26967: f64, t26977: f64, t27170: f64, t4028: f64, t4072: f64, t5113: f64, t671: f64, t7042: f64, t7056: f64, t7676: f64, t7801: f64, t1774: f64, t1266: f64, t1442: f64, t1459: f64, t2036: f64, t2040: f64, t2075: f64, t4026: f64, t4034: f64, t4073: f64, t4077: f64, t5107: f64, t574: f64, t652: f64, t672: f64, t7040: f64, t7156: f64, t7787: f64, t7802: f64, t26895: f64, t26982: f64, t27183: f64, t3: f64, t112: f64, t7945: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t20173: f64, t24462: f64, t24465: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64, t7230: f64, t7235: f64, t7956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t27188 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1750(t111, t7786);
        let t27215 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1751(t1268, t12725, t1458, t19456, t2039, t2314, t23938, t26114, t26117, t26967, t26977, t27170, t27188, t4028, t4072, t5113, t671, t7042, t7056, t7676, t7801);
        let (t27219, t27226, t27238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1752(t1774, t7056, t1266, t7801, t12725, t1442, t1459, t2036, t2040, t2075, t2314, t23938, t27188, t27215, t4026, t4034, t4073, t4077, t5107, t574, t652, t672, t7040, t7042, t7156, t7787, t7802);
        let (t27240, t27241, t27254, t27273, t27276, t27281) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1753(t26895, t26982, t27183, t27238, t3, t112, t7945, t1458, t7056, t2039, t4072, t671, t7801);
        let t27286 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1754(t12524, t1401, t1458, t16521, t16524, t20173, t2039, t24462, t24465, t27170, t27240, t27254, t27273, t27276, t27281, t3938, t3941, t4072, t5371, t5376, t577, t671, t7056, t7230, t7235, t7801, t7956);
    (t27188, t27215, t27219, t27226, t27240, t27241, t27254, t27273, t27276, t27281, t27286)
}
