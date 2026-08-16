//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta384 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1483;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1484;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1485;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1486;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1487;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1488;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1489;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1490;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta384(t1735: f64, t3252: f64, t3578: f64, t3248: f64, t11642: f64, t11644: f64, t11649: f64, t1174: f64, t1227: f64, t15434: f64, t15438: f64, t15446: f64, t15448: f64, t15450: f64, t15452: f64, t15455: f64, t3518: f64, t3527: f64, t3531: f64, t3577: f64, t5005: f64, t1216: f64, t4733: f64, t1653: f64, t3494: f64, t1090: f64, t5012: f64, t3490: f64, t4993: f64, t248: f64, t3521: f64, t3536: f64, t4997: f64, t3570: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11665: f64, t1218: f64, t1232: f64, t3511: f64, t3587: f64, t4954: f64, t5024: f64, t11539: f64, t4724: f64, t15239: f64, t475: f64, t1214: f64, t4977: f64, t4582: f64, t3516: f64, t12652: f64, t4987: f64, t12648: f64, t13969: f64, t4983: f64, t3515: f64, t486: f64, t5011: f64, t4978: f64, t11709: f64, t11738: f64, t11814: f64, t11825: f64, t1737: f64, t1748: f64, t3506: f64, t4980: f64, t4989: f64, t5014: f64, t3509: f64, t478: f64, t3068: f64, t1244: f64, t11697: f64, t4949: f64, t3431: f64, t4729: f64, t1177: f64, t14749: f64, t14753: f64, t14744: f64, t1011: f64, t15031: f64, t1212: f64, t1226: f64, t4965: f64, t11652: f64, t11678: f64, t11692: f64, t11699: f64, t11703: f64, t3496: f64, t3580: f64, t4950: f64, t5002: f64, t4953: f64, t4972: f64, t1229: f64, t3242: f64, t14165: f64, t3493: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t15466 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1483(t1735, t3252, t3578, t3248, t11642, t11644, t11649, t1174, t1227, t15434, t15438, t15446, t15448, t15450, t15452, t15455, t3518, t3527, t3531, t3577, t5005);
        let (t15470, t15474, t15478, t15484, t15488) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1484(t1216, t4733, t3578, t1653, t3494, t1090, t5012, t3490, t4993, t248, t3521, t1227);
        let (t15490, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1485(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let t15512 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1486(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
        let (t15524, t15527, t15531, t15535, t15540) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1487(t11539, t4724, t1174, t15239, t475, t1214, t248, t3494, t4977, t4582, t3516, t12652, t4987);
        let (t15553, t15558) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1488(t15540, t4582, t12648, t4987, t13969, t4983, t3515, t486, t5011, t4978, t11709, t11738, t11814, t11825, t1213, t1227, t15524, t15527, t15531, t15535, t1737, t1748, t3490, t3506, t3531, t3536, t4980, t4989, t5014, t5024);
        let (t15560, t15564, t15569, t15574, t15578) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1489(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244, t11697, t4949, t3577, t3431, t4729);
        let t15601 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1490(t1174, t15578, t1177, t14749, t14753, t14744, t1011, t15031, t1212, t1226, t4965, t11652, t11665, t11678, t11692, t11699, t11703, t1218, t1232, t15560, t15564, t15569, t15574, t3496, t3580, t4950, t5002);
        let (t15610, t15612, t15617, t15621) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1491(t11697, t4953, t3577, t12648, t4972, t4582, t1229, t3242, t14165, t3493, t3508, t4977);
    (t15466, t15512, t15553, t15558, t15601, t15610, t15612, t15617, t15621)
}
