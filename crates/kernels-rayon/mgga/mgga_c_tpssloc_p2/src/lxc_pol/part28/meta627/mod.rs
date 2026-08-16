//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1956;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1957;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1958;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1959;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1960;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1961;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1962;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1963;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1965;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta627(t26959: f64, t6495: f64, t26070: f64, t7032: f64, t26073: f64, t26076: f64, t23998: f64, t7435: f64, t23967: f64, t26090: f64, t23993: f64, t46104: f64, t7025: f64, t26055: f64, t22531: f64, t22537: f64, t23963: f64, t26911: f64, t6492: f64, t7782: f64, t90196: f64, t26063: f64, t7432: f64, t84241: f64, t2032: f64, t22493: f64, t24001: f64, t26009: f64, t26028: f64, t32332: f64, t7035: f64, t7428: f64, t84222: f64, t84224: f64, t84229: f64, t84245: f64, t90205: f64, t9239: f64, t45844: f64, t12571: f64, t23966: f64, t84195: f64, t1860: f64, t2031: f64, t22527: f64, t22546: f64, t23975: f64, t26067: f64, t26945: f64, t6486: f64, t7026: f64, t84209: f64, t90202: f64, t90227: f64, t90232: f64, t90257: f64, t22519: f64, t90150: f64, t90177: f64, t90334: f64, t90337: f64, t90340: f64, t90343: f64, t23992: f64, t7445: f64, t26016: f64, t84173: f64, t22534: f64, t23970: f64, t84237: f64, t90098: f64, t90101: f64, t90104: f64, t90132: f64, t90137: f64, t90153: f64, t26024: f64, t7031: f64, t84180: f64, t84216: f64, t84242: f64, t84248: f64, t84270: f64, t84280: f64, t84283: f64, t84285: f64, t90072: f64, t90121: f64, t90141: f64, t90090: f64, t26012: f64, t22549: f64, t90094: f64, t84219: f64, t90247: f64, t26954: f64, t83722: f64, t83778: f64, t84183: f64, t84190: f64, t90076: f64, t90080: f64, t90114: f64, t5: f64, t91888: f64, t112: f64, t111: f64, t26966: f64, t12813: f64, t1458: f64, t2039: f64, t2363: f64, t23917: f64, t23938: f64, t26977: f64, t27188: f64, t4028: f64, t4072: f64, t45632: f64, t55962: f64, t671: f64, t7042: f64, t84097: f64, t90381: f64, t91854: f64, t91857: f64, t91870: f64, t109: f64, t86586: f64, t86588: f64, t86590: f64, t81440: f64, t81443: f64, t81445: f64, t84036: f64, t86593: f64, t86596: f64, t86599: f64, t86601: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t19456: f64, t2314: f64, t26114: f64, t26117: f64, t27170: f64, t5113: f64, t55934: f64, t7056: f64, t7676: f64, t7801: f64, t90370: f64, t90375: f64, t9348: f64, t12835: f64, t1459: f64, t15857: f64, t15868: f64, t1774: f64, t1983: f64, t2040: f64, t2095: f64, t23909: f64, t23918: f64, t24432: f64, t24987: f64, t24995: f64, t26179: f64, t26872: f64, t4037: f64, t4077: f64, t55169: f64, t574: f64, t652: f64, t7057: f64, t7217: f64, t7220: f64, t7458: f64, t7802: f64, t83886: f64, t86685: f64, t2094: f64, t40611: f64, t12461: f64, t7216: f64, t1266: f64, t22574: f64, t2323: f64, t23857: f64, t23933: f64, t24169: f64, t24433: f64, t25988: f64, t26161: f64, t26163: f64, t26870: f64, t26902: f64, t26906: f64, t27147: f64, t27171: f64, t27180: f64, t32193: f64, t34711: f64, t4034: f64, t510: f64, t5308: f64, t6876: f64, t7685: f64, t7806: f64, t7940: f64, t91655: f64, t91687: f64) -> (f64, f64, f64, f64, f64) {
        let (t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1955(t26959, t6495, t26070, t7032, t26073, t26076, t23998, t7435, t23967, t26090, t23993, t46104, t7025);
        let t91914 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1956(t26055, t7032, t22531, t22537, t23963, t26911, t6492, t7782, t90196, t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907);
        let t91938 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1957(t23967, t26063, t7432, t84241, t2032, t22493, t24001, t26009, t26028, t26073, t32332, t7035, t7428, t7782, t84222, t84224, t84229, t84245, t90205, t9239);
        let t91966 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1958(t45844, t7025, t12571, t23966, t6492, t7432, t84195, t1860, t2031, t2032, t22527, t22546, t23975, t26063, t26067, t26911, t26945, t6486, t7026, t84209, t90202, t90227, t90232, t90257);
        let t91993 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1959(t23967, t26067, t2032, t22519, t23975, t26055, t26070, t26090, t26945, t6495, t7026, t7035, t7782, t90150, t90177, t90334, t90337, t90340, t90343);
        let t92019 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1960(t23993, t7428, t23998, t1860, t23992, t7445, t26028, t7032, t26016, t84173, t2032, t22534, t23970, t7782, t84237, t90098, t90101, t90104, t90132, t90137, t90153);
        let t92039 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1961(t26959, t6486, t1860, t26024, t7031, t2032, t23963, t26016, t84180, t84216, t84242, t84248, t84270, t84280, t84283, t84285, t90072, t90121, t90141);
        let t92068 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1962(t2031, t90090, t26012, t7031, t22549, t90094, t26009, t84219, t90247, t23963, t23970, t26016, t26954, t83722, t83778, t84183, t84190, t90076, t90080, t90114);
        let (t92073, t92090, t92099) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1963(t5, t91888, t91914, t91938, t91966, t91993, t92019, t92039, t92068, t112, t111, t26966, t12813, t1458, t2039, t2363, t23917, t23938, t26977, t27188, t4028, t4072, t45632, t55962, t671, t7042, t84097, t90381, t91854, t91857, t91870);
        let (t92128, t92139) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964(t109, t86586, t86588, t86590, t81440, t81443, t81445, t84036, t86593, t86596, t86599, t86601, t1268, t12725, t12734, t12739, t19456, t2039, t2314, t23917, t26114, t26117, t27170, t5113, t55934, t7056, t7676, t7801, t90370, t90375, t9348);
        let t92161 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1965(t12835, t1459, t15857, t15868, t1774, t19456, t1983, t2039, t2040, t2095, t23909, t23917, t23918, t23938, t24432, t24987, t24995, t26179, t26872, t26977, t4028, t4037, t4077, t55169, t574, t652, t7042, t7057, t7217, t7220, t7458, t7802, t83886, t86685, t90381, t91854, t91857, t92099, t92139, t9348);
        let t92210 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1966(t2094, t40611, t12461, t7216, t1266, t12734, t1983, t22574, t2314, t2323, t23857, t23933, t24169, t24433, t24995, t25988, t26161, t26163, t26870, t26902, t26906, t27147, t27170, t27171, t27180, t27188, t32193, t34711, t4028, t4034, t510, t5308, t652, t671, t6876, t7685, t7806, t7940, t91655, t91687, t92128);
    (t92073, t92090, t92128, t92161, t92210)
}
