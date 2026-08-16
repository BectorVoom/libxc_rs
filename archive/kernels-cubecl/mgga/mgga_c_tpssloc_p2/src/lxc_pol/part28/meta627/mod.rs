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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta627<F: Float>(t26959: F, t6495: F, t26070: F, t7032: F, t26073: F, t26076: F, t23998: F, t7435: F, t23967: F, t26090: F, t23993: F, t46104: F, t7025: F, t26055: F, t22531: F, t22537: F, t23963: F, t26911: F, t6492: F, t7782: F, t90196: F, t26063: F, t7432: F, t84241: F, t2032: F, t22493: F, t24001: F, t26009: F, t26028: F, t32332: F, t7035: F, t7428: F, t84222: F, t84224: F, t84229: F, t84245: F, t90205: F, t9239: F, t45844: F, t12571: F, t23966: F, t84195: F, t1860: F, t2031: F, t22527: F, t22546: F, t23975: F, t26067: F, t26945: F, t6486: F, t7026: F, t84209: F, t90202: F, t90227: F, t90232: F, t90257: F, t22519: F, t90150: F, t90177: F, t90334: F, t90337: F, t90340: F, t90343: F, t23992: F, t7445: F, t26016: F, t84173: F, t22534: F, t23970: F, t84237: F, t90098: F, t90101: F, t90104: F, t90132: F, t90137: F, t90153: F, t26024: F, t7031: F, t84180: F, t84216: F, t84242: F, t84248: F, t84270: F, t84280: F, t84283: F, t84285: F, t90072: F, t90121: F, t90141: F, t90090: F, t26012: F, t22549: F, t90094: F, t84219: F, t90247: F, t26954: F, t83722: F, t83778: F, t84183: F, t84190: F, t90076: F, t90080: F, t90114: F, t5: F, t91888: F, t112: F, t111: F, t26966: F, t12813: F, t1458: F, t2039: F, t2363: F, t23917: F, t23938: F, t26977: F, t27188: F, t4028: F, t4072: F, t45632: F, t55962: F, t671: F, t7042: F, t84097: F, t90381: F, t91854: F, t91857: F, t91870: F, t109: F, t86586: F, t86588: F, t86590: F, t81440: F, t81443: F, t81445: F, t84036: F, t86593: F, t86596: F, t86599: F, t86601: F, t1268: F, t12725: F, t12734: F, t12739: F, t19456: F, t2314: F, t26114: F, t26117: F, t27170: F, t5113: F, t55934: F, t7056: F, t7676: F, t7801: F, t90370: F, t90375: F, t9348: F, t12835: F, t1459: F, t15857: F, t15868: F, t1774: F, t1983: F, t2040: F, t2095: F, t23909: F, t23918: F, t24432: F, t24987: F, t24995: F, t26179: F, t26872: F, t4037: F, t4077: F, t55169: F, t574: F, t652: F, t7057: F, t7217: F, t7220: F, t7458: F, t7802: F, t83886: F, t86685: F, t2094: F, t40611: F, t12461: F, t7216: F, t1266: F, t22574: F, t2323: F, t23857: F, t23933: F, t24169: F, t24433: F, t25988: F, t26161: F, t26163: F, t26870: F, t26902: F, t26906: F, t27147: F, t27171: F, t27180: F, t32193: F, t34711: F, t4034: F, t510: F, t5308: F, t6876: F, t7685: F, t7806: F, t7940: F, t91655: F, t91687: F) -> (F, F, F, F, F) {
        let (t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1955::<F>(t26959, t6495, t26070, t7032, t26073, t26076, t23998, t7435, t23967, t26090, t23993, t46104, t7025);
        let t91914 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1956::<F>(t26055, t7032, t22531, t22537, t23963, t26911, t6492, t7782, t90196, t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907);
        let t91938 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1957::<F>(t23967, t26063, t7432, t84241, t2032, t22493, t24001, t26009, t26028, t26073, t32332, t7035, t7428, t7782, t84222, t84224, t84229, t84245, t90205, t9239);
        let t91966 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1958::<F>(t45844, t7025, t12571, t23966, t6492, t7432, t84195, t1860, t2031, t2032, t22527, t22546, t23975, t26063, t26067, t26911, t26945, t6486, t7026, t84209, t90202, t90227, t90232, t90257);
        let t91993 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1959::<F>(t23967, t26067, t2032, t22519, t23975, t26055, t26070, t26090, t26945, t6495, t7026, t7035, t7782, t90150, t90177, t90334, t90337, t90340, t90343);
        let t92019 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1960::<F>(t23993, t7428, t23998, t1860, t23992, t7445, t26028, t7032, t26016, t84173, t2032, t22534, t23970, t7782, t84237, t90098, t90101, t90104, t90132, t90137, t90153);
        let t92039 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1961::<F>(t26959, t6486, t1860, t26024, t7031, t2032, t23963, t26016, t84180, t84216, t84242, t84248, t84270, t84280, t84283, t84285, t90072, t90121, t90141);
        let t92068 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1962::<F>(t2031, t90090, t26012, t7031, t22549, t90094, t26009, t84219, t90247, t23963, t23970, t26016, t26954, t83722, t83778, t84183, t84190, t90076, t90080, t90114);
        let (t92073, t92090, t92099) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1963::<F>(t5, t91888, t91914, t91938, t91966, t91993, t92019, t92039, t92068, t112, t111, t26966, t12813, t1458, t2039, t2363, t23917, t23938, t26977, t27188, t4028, t4072, t45632, t55962, t671, t7042, t84097, t90381, t91854, t91857, t91870);
        let (t92128, t92139) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964::<F>(t109, t86586, t86588, t86590, t81440, t81443, t81445, t84036, t86593, t86596, t86599, t86601, t1268, t12725, t12734, t12739, t19456, t2039, t2314, t23917, t26114, t26117, t27170, t5113, t55934, t7056, t7676, t7801, t90370, t90375, t9348);
        let t92161 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1965::<F>(t12835, t1459, t15857, t15868, t1774, t19456, t1983, t2039, t2040, t2095, t23909, t23917, t23918, t23938, t24432, t24987, t24995, t26179, t26872, t26977, t4028, t4037, t4077, t55169, t574, t652, t7042, t7057, t7217, t7220, t7458, t7802, t83886, t86685, t90381, t91854, t91857, t92099, t92139, t9348);
        let t92210 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1966::<F>(t2094, t40611, t12461, t7216, t1266, t12734, t1983, t22574, t2314, t2323, t23857, t23933, t24169, t24433, t24995, t25988, t26161, t26163, t26870, t26902, t26906, t27147, t27170, t27171, t27180, t27188, t32193, t34711, t4028, t4034, t510, t5308, t652, t671, t6876, t7685, t7806, t7940, t91655, t91687, t92128);
    (t92073, t92090, t92128, t92161, t92210)
}
