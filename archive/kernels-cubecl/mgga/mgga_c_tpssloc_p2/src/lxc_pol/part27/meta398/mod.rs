//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta398 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1642;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1643;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1644;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1645;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1646;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1647;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1648;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1649;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta398<F: Float>(t1735: F, t3252: F, t3578: F, t3248: F, t11642: F, t11644: F, t11649: F, t1174: F, t1227: F, t15434: F, t15438: F, t15446: F, t15448: F, t15450: F, t15452: F, t15455: F, t3518: F, t3527: F, t3531: F, t3577: F, t5005: F, t1216: F, t4733: F, t1653: F, t3494: F, t1090: F, t5012: F, t3490: F, t4993: F, t248: F, t3521: F, t3536: F, t4997: F, t3570: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F, t3503: F, t3500: F, t1210: F, t11665: F, t1218: F, t1232: F, t3511: F, t3587: F, t4954: F, t5024: F, t11539: F, t4724: F, t15239: F, t475: F, t1214: F, t4977: F, t4582: F, t3516: F, t12652: F, t4987: F, t12648: F, t13969: F, t4983: F, t3515: F, t486: F, t5011: F, t4978: F, t11709: F, t11738: F, t11814: F, t11825: F, t1737: F, t1748: F, t3506: F, t4980: F, t4989: F, t5014: F, t3509: F, t478: F, t3068: F, t1244: F, t11697: F, t4949: F, t3431: F, t4729: F, t1177: F, t14749: F, t14753: F, t14744: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t11652: F, t11678: F, t11692: F, t11699: F, t11703: F, t3496: F, t3580: F, t4950: F, t5002: F, t4953: F, t4972: F, t1229: F, t3242: F, t14165: F, t3493: F, t3508: F) -> (F, F, F, F, F, F, F, F, F) {
        let t15466 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1642::<F>(t1735, t3252, t3578, t3248, t11642, t11644, t11649, t1174, t1227, t15434, t15438, t15446, t15448, t15450, t15452, t15455, t3518, t3527, t3531, t3577, t5005);
        let (t15470, t15474, t15478, t15484, t15488) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1643::<F>(t1216, t4733, t3578, t1653, t3494, t1090, t5012, t3490, t4993, t248, t3521, t1227);
        let (t15490, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1644::<F>(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let t15512 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1645::<F>(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
        let (t15524, t15527, t15531, t15535, t15540) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1646::<F>(t11539, t4724, t1174, t15239, t475, t1214, t248, t3494, t4977, t4582, t3516, t12652, t4987);
        let (t15553, t15558) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1647::<F>(t15540, t4582, t12648, t4987, t13969, t4983, t3515, t486, t5011, t4978, t11709, t11738, t11814, t11825, t1213, t1227, t15524, t15527, t15531, t15535, t1737, t1748, t3490, t3506, t3531, t3536, t4980, t4989, t5014, t5024);
        let (t15560, t15564, t15569, t15574, t15578) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1648::<F>(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244, t11697, t4949, t3577, t3431, t4729);
        let t15601 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1649::<F>(t1174, t15578, t1177, t14749, t14753, t14744, t1011, t15031, t1212, t1226, t4965, t11652, t11665, t11678, t11692, t11699, t11703, t1218, t1232, t15560, t15564, t15569, t15574, t3496, t3580, t4950, t5002);
        let (t15610, t15612, t15617, t15621) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1650::<F>(t11697, t4953, t3577, t12648, t4972, t4582, t1229, t3242, t14165, t3493, t3508, t4977);
    (t15466, t15512, t15553, t15558, t15601, t15610, t15612, t15617, t15621)
}
