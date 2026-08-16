//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1169;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1170;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1171;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1172;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1173;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta328<F: Float>(t39582: F, t39585: F, t39590: F, t39593: F, t39595: F, t39597: F, t39602: F, t39604: F, t39606: F, t39608: F, t39610: F, t39612: F, t39615: F, t39621: F, t39629: F, t39631: F, t39633: F, t39635: F, t39637: F, t39640: F, t39643: F, t39645: F, t39655: F, t39658: F, t39660: F, t12126: F, t588: F, t39037: F, t522: F, t2221: F, t3826: F, t3824: F, t12132: F, t592: F, t3696: F, t2223: F, t39844: F, t39846: F, t39852: F, t39854: F, t39856: F, t39858: F, t68: F, t6924: F, t12012: F, t12147: F, t12157: F, t12160: F, t12161: F, t12164: F, t1345: F, t1347: F, t1348: F, t16186: F, t1995: F, t225: F, t3719: F, t3734: F, t3839: F, t3843: F, t3844: F, t3847: F, t39622: F, t39892: F, t40026: F, t40210: F, t40211: F, t40213: F, t40214: F, t40217: F, t5278: F, t546: F, t548: F, t550: F, t1336: F, t1339: F, t2691: F, t3809: F, t12267: F, t3865: F, t1369: F, t1362: F, t40118: F, t12344: F, t3777: F, t12361: F, t3866: F, t12336: F, t12379: F, t12392: F, t12397: F, t12404: F, t12429: F, t1341: F, t1343: F, t1363: F, t1367: F, t3778: F, t3858: F, t3876: F, t40206: F, t820: F) -> (F, F, F, F, F, F, F, F, F) {
        let t40218 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1169::<F>(t39582, t39585, t39590, t39593, t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615);
        let t40220 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1170::<F>(t39621, t39629, t39631, t39633, t39635, t39637, t39640, t39643, t39645, t39655, t39658, t39660);
        let (t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40235) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1171::<F>(t12126, t588, t39037, t522, t2221, t3826, t3824, t12132, t592, t3696, t2223, t39844, t39846, t39852, t39854, t39856, t39858);
        let t40270 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1172::<F>(t68, t6924, t12012, t12147, t12157, t12160, t12161, t12164, t1345, t1347, t1348, t16186, t1995, t225, t3719, t3734, t3839, t3843, t3844, t3847, t39622, t39892, t40026, t40210, t40211, t40213, t40214, t40217, t40218, t40220, t40235, t5278, t546, t548);
        let (t40271, t40282, t40285, t40287, t40292) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1173::<F>(t40270, t550, t1336, t1339, t2691, t3809, t12267, t3865, t1369, t1362, t40118, t12344, t3777);
        let t40303 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174::<F>(t1369, t40292, t12361, t3866, t12336, t12379, t12392, t12397, t12404, t12429, t1341, t1343, t1363, t1367, t3778, t3858, t3876, t39892, t40206, t40271, t40282, t40285, t40287, t820);
    (t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40271, t40303)
}
