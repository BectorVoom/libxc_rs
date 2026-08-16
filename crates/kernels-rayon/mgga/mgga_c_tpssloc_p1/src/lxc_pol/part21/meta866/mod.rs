//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta866 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta866(t65278: f64, t65279: f64, t65281: f64, t65282: f64, t65285: f64, t65286: f64, t65297: f64, t65327: f64, t11881: f64, t11907: f64, t1235: f64, t1244: f64, t1246: f64, t14986: f64, t15000: f64, t15009: f64, t15027: f64, t15239: f64, t1755: f64, t18940: f64, t19128: f64, t19160: f64, t19179: f64, t3610: f64, t3612: f64, t3613: f64, t3624: f64, t3626: f64, t491: f64, t5064: f64, t5079: f64, t6260: f64, t65221: f64, t65254: f64, t65262: f64, t65265: f64, t3590: f64, t6224: f64, t11877: f64, t11904: f64, t14989: f64, t15004: f64, t15032: f64, t15248: f64, t19123: f64, t19139: f64, t19189: f64, t19201: f64, t19204: f64, t3617: f64, t3625: f64, t5011: f64, t5052: f64, t5080: f64, t5084: f64, t52435: f64, t53565: f64, t6261: f64, t1215: f64, t19120: f64, t19131: f64, t19145: f64, t19146: f64, t19154: f64, t19165: f64, t19176: f64, t3493: f64, t3507: f64, t3621: f64, t44753: f64, t44754: f64, t45329: f64, t5068: f64, t5069: f64, t52485: f64, t6238: f64, t6252: f64, t6257: f64, t1213: f64, t18941: f64, t248: f64, t3570: f64, t15730: f64, t5019: f64, t1216: f64, t3966: f64, t1227: f64, t1230: f64, t15495: f64, t15498: f64, t15708: f64, t15710: f64, t15740: f64, t1737: f64, t1748: f64, t19051: f64, t3527: f64, t3531: f64, t3577: f64, t3578: f64, t3585: f64, t44929: f64, t44932: f64, t4728: f64, t5014: f64, t5030: f64, t53406: f64, t53507: f64, t5971: f64, t6227: f64, t6232: f64, t63357: f64, t63363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t65330, t65343) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160(t65278, t65279, t65281, t65282, t65285, t65286, t65297, t65327, t11881, t11907, t1235, t1244, t1246, t14986, t15000, t15009, t15027, t15239, t1755, t18940, t19128, t19160, t19179, t3610, t3612, t3613, t3624, t3626, t491, t5064, t5079, t6260, t65221, t65254, t65262, t65265);
        let (t65347, t65374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161(t3590, t6224, t11877, t11904, t11907, t1244, t1246, t14989, t15004, t15027, t15032, t15248, t19123, t19139, t19189, t19201, t19204, t3617, t3624, t3625, t5011, t5052, t5064, t5079, t5080, t5084, t52435, t53565, t6261);
        let t65408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162(t11877, t11881, t11904, t11907, t1215, t1244, t1246, t19120, t19128, t19131, t19145, t19146, t19154, t19165, t19176, t19189, t19201, t3493, t3507, t3610, t3621, t44753, t44754, t45329, t5068, t5069, t52485, t6238, t6252, t6257);
        let (t65452, t65463) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163(t1213, t18941, t248, t3570, t15730, t5019, t1216, t3966, t1227, t1230, t15495, t15498, t15708, t15710, t15740, t1737, t1748, t19051, t3527, t3531, t3577, t3578, t3585, t44929, t44932, t4728, t5014, t5030, t53406, t53507, t5971, t6227, t6232, t63357, t63363);
    (t65330, t65343, t65347, t65374, t65408, t65452, t65463)
}
