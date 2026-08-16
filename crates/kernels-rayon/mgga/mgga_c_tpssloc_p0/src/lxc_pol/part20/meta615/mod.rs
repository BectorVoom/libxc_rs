//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2216;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2218;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta615(t2250: f64, t3966: f64, t12606: f64, t607: f64, t12648: f64, t12649: f64, t12652: f64, t12653: f64, t12661: f64, t12709: f64, t1434: f64, t2252: f64, t31: f64, t4018: f64, t45872: f64, t45993: f64, t628: f64, t642: f64, t65: f64, t80: f64, t9263: f64, t12623: f64, t12656: f64, t12662: f64, t12665: f64, t1411: f64, t1426: f64, t2251: f64, t2304: f64, t3962: f64, t3968: f64, t3971: f64, t3997: f64, t67: f64, t9248: f64, t9259: f64, t9339: f64, t12620: f64, t12633: f64, t12636: f64, t12708: f64, t1410: f64, t2255: f64, t2283: f64, t3961: f64, t3967: f64, t3976: f64, t608: f64, t609: f64, t7445: f64, t9247: f64, t9260: f64, t9268: f64, t9312: f64, t16: f64, t39031: f64, t39033: f64, t39035: f64, t39037: f64, t39039: f64, t12566: f64, t604: f64, t2239: f64, t3951: f64, t12571: f64, t12582: f64, t12719: f64, t1437: f64, t2240: f64, t2241: f64, t39043: f64, t39049: f64, t39054: f64, t3953: f64, t3958: f64, t4021: f64, t45986: f64, t605: f64, t645: f64, t86: f64, t9239: f64, t9243: f64, t9342: f64, t5: f64, t12568: f64, t12585: f64, t12588: f64, t2235: f64, t2307: f64, t39046: f64, t39063: f64, t45844: f64, t9228: f64, t9231: f64, t9240: f64, t112: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t19456: f64, t2314: f64, t2363: f64, t39235: f64, t4028: f64, t4072: f64, t45590: f64, t45602: f64, t45632: f64, t45637: f64, t45782: f64, t45814: f64, t5113: f64, t671: f64, t9348: f64, t9416: f64, t40626: f64, t4199: f64, t9919: f64, t12887: f64, t758: f64, t9892: f64, t13123: f64, t9882: f64, t9888: f64, t118: f64, t2375: f64, t4095: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t45997 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2216(t2250, t3966);
        let (t46006, t46022) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217(t12606, t607, t12648, t12649, t12652, t12653, t12661, t12709, t1434, t2252, t31, t4018, t45872, t45993, t45997, t628, t642, t65, t80, t9263);
        let t46050 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2218(t12623, t12656, t12662, t12665, t1411, t1426, t2251, t2304, t3962, t3968, t3971, t3997, t607, t642, t67, t80, t9248, t9259, t9339);
        let t46080 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219(t12620, t12633, t12636, t12708, t1410, t1434, t2250, t2255, t2283, t2304, t3961, t3967, t3976, t4018, t608, t609, t642, t7445, t80, t9247, t9260, t9268, t9312);
        let t46114 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220(t16, t39031, t39033, t39035, t39037, t39039, t12566, t604, t2239, t3951, t12571, t12582, t12719, t1437, t2240, t2241, t39043, t39049, t39054, t3953, t3958, t4021, t45986, t46022, t46050, t46080, t605, t645, t86, t9239, t9243, t9342);
        let t46116 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221(t5, t12568, t12585, t12588, t12719, t1437, t2235, t2240, t2307, t39046, t39063, t3958, t4021, t45844, t46114, t9228, t9231, t9239, t9240);
        let (t46117, t46118) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222(t112, t46116, t1268, t12725, t12734, t12739, t12813, t1458, t19456, t2314, t2363, t39235, t4028, t4072, t45590, t45602, t45632, t45637, t45782, t45814, t5113, t671, t9348, t9416);
        let (t46120, t46126, t46129, t46131, t46133, t46135, t46137) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223(t40626, t4199, t9919, t12887, t67, t758, t9892, t13123, t9882, t9888, t118, t2375, t4095);
    (t45997, t46006, t46117, t46118, t46120, t46126, t46129, t46131, t46133, t46135, t46137)
}
