//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1169;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1170;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1171;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1172;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1173;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta328(t39582: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t39597: f64, t39602: f64, t39604: f64, t39606: f64, t39608: f64, t39610: f64, t39612: f64, t39615: f64, t39621: f64, t39629: f64, t39631: f64, t39633: f64, t39635: f64, t39637: f64, t39640: f64, t39643: f64, t39645: f64, t39655: f64, t39658: f64, t39660: f64, t12126: f64, t588: f64, t39037: f64, t522: f64, t2221: f64, t3826: f64, t3824: f64, t12132: f64, t592: f64, t3696: f64, t2223: f64, t39844: f64, t39846: f64, t39852: f64, t39854: f64, t39856: f64, t39858: f64, t68: f64, t6924: f64, t12012: f64, t12147: f64, t12157: f64, t12160: f64, t12161: f64, t12164: f64, t1345: f64, t1347: f64, t1348: f64, t16186: f64, t1995: f64, t225: f64, t3719: f64, t3734: f64, t3839: f64, t3843: f64, t3844: f64, t3847: f64, t39622: f64, t39892: f64, t40026: f64, t40210: f64, t40211: f64, t40213: f64, t40214: f64, t40217: f64, t5278: f64, t546: f64, t548: f64, t550: f64, t1336: f64, t1339: f64, t2691: f64, t3809: f64, t12267: f64, t3865: f64, t1369: f64, t1362: f64, t40118: f64, t12344: f64, t3777: f64, t12361: f64, t3866: f64, t12336: f64, t12379: f64, t12392: f64, t12397: f64, t12404: f64, t12429: f64, t1341: f64, t1343: f64, t1363: f64, t1367: f64, t3778: f64, t3858: f64, t3876: f64, t40206: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t40218 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1169(t39582, t39585, t39590, t39593, t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615);
        let t40220 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1170(t39621, t39629, t39631, t39633, t39635, t39637, t39640, t39643, t39645, t39655, t39658, t39660);
        let (t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40235) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1171(t12126, t588, t39037, t522, t2221, t3826, t3824, t12132, t592, t3696, t2223, t39844, t39846, t39852, t39854, t39856, t39858);
        let t40270 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1172(t68, t6924, t12012, t12147, t12157, t12160, t12161, t12164, t1345, t1347, t1348, t16186, t1995, t225, t3719, t3734, t3839, t3843, t3844, t3847, t39622, t39892, t40026, t40210, t40211, t40213, t40214, t40217, t40218, t40220, t40235, t5278, t546, t548);
        let (t40271, t40282, t40285, t40287, t40292) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1173(t40270, t550, t1336, t1339, t2691, t3809, t12267, t3865, t1369, t1362, t40118, t12344, t3777);
        let t40303 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1174(t1369, t40292, t12361, t3866, t12336, t12379, t12392, t12397, t12404, t12429, t1341, t1343, t1363, t1367, t3778, t3858, t3876, t39892, t40206, t40271, t40282, t40285, t40287, t820);
    (t40222, t40224, t40226, t40228, t40230, t40232, t40234, t40271, t40303)
}
